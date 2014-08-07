use arena::TypedArena;
use ft = freetype;
use graphics::{
    AddImage,
    AddColor,
    ColorContext,
    Context,
    Draw,
    ImageSize,
    RelativeTransform2d,
};
use nalgebra::na;
use nalgebra::na::{Vec1, Vec2, Iso2, Translation, Rotation};
use ncollide::geom;
use ncollide::geom::Cuboid;
use nphysics::object::RigidBody;
use nphysics::world::World;
use opengl_graphics::{
    Gl,
    Texture,
};
use std::any::AnyRefExt;
use std::cell::{RefCell, UnsafeCell};
use std::collections::hashmap::HashMap;
use std::io::stdio;
use std::kinds::marker;
use std::num::One;
use std::rc::Rc;

struct Letter {
    texture: Texture,
    left: i32,
    top: i32,
    advance: ft::Vector,
}

pub struct WordContext<'dict, 'tree> {
    library: ft::Library,
    face: Option<ft::Face>,
    arena: TypedArena<Letter>,
    letters: UnsafeCell<HashMap<char, &'tree Letter>>,
    marker: marker::NoShare
}

impl<'dict, 'tree> WordContext<'dict, 'tree> {
    pub fn make<'dict, 'tree>() -> WordContext<'dict, 'tree> {
        let freetype = ft::Library::init().unwrap();
        static font: &'static str = "/usr/share/fonts/TTF/DejaVuSans.ttf";
        let face = freetype.new_face(font, 0).unwrap();
        face.set_pixel_sizes(0, 48).unwrap();
        WordContext {
            library: freetype,
            face: Some(face),
            arena: TypedArena::new(),
            letters: UnsafeCell::new(HashMap::new()),
            marker: marker::NoShare,
        }
    }

    fn letter(&'tree self, ch: char) -> &Letter {
        let inserter = |ch: &char| {
            // This unwrap is safe because face is only None after WordContext is dropped.
            let face = self.face.as_ref().unwrap();
            face.load_char(*ch as u64, ft::face::Render).unwrap();
            let g = face.glyph();
            let t = Texture::from_memory_alpha(
                g.bitmap().buffer(),
                g.bitmap().width() as u32,
                g.bitmap().rows() as u32).unwrap();
            self.arena.alloc(Letter {
                texture: t,
                left: g.bitmap_left(),
                top: g.bitmap_top(),
                advance: g.advance(),
            })
        };
        let ref letters = self.letters;
        *unsafe {
            // There can be no data races since WordContext is not Share.  We do not access the
            // hash table outside of this block.
            (*letters.get()).find_or_insert_with(ch, inserter)
        }
    }
}

#[unsafe_destructor]
impl<'dict, 'tree> Drop for WordContext<'dict, 'tree> {
    fn drop(&mut self) {
        self.face = None;
        unsafe {
            let err = ft::ffi::FT_Done_FreeType(self.library.raw());
            if err != 0 {
                stdio::println(format!("Failed to drop Library. Error Code: {}", err).as_slice());
            }
        }
    }
}

pub struct WordBox<'tree> {
    ctx: ColorContext,
    word: Vec<&'tree Letter>,
    body: Rc<RefCell<RigidBody>>,
}

impl<'tree> WordBox<'tree> {
    // Precondition: word.len() > 0
    pub fn make<'dict, 'tree>(ctx: &Context, word_ctx: &'tree WordContext<'dict, 'tree>, world: &mut World, scale: f64, word: &'dict str) -> WordBox<'tree> {
        assert!(word.len() > 0);
        let vec: Vec<&Letter> = word.chars().map( |ch| word_ctx.letter(ch) ).collect();
        let (x_px, y_px, (width_px, height_px)) = {
            // Unwrap is safe because we know word was non-empty
            let first_letter = vec.iter().next().unwrap();
            let mut iter = vec.iter().rev();
            // Unwrap is safe because we know word was non-empty
            let last_letter = iter.next().unwrap();
            let x = first_letter.left;
            let (_, fh) = first_letter.texture.get_size();
            let y = fh as i32 - first_letter.top;
            let (w, h) = last_letter.texture.get_size();
            (x, y, iter.fold( (w as i64, h as i64), |(w, h), l|
                (w + (l.advance.x >> 6), h + (l.advance.y >> 6)) ))
        };
        let width = width_px as f64 / scale / 2.0;
        let height = height_px as f64 / scale / 2.0;
        let x = x_px as f64 / scale + width;
        let y = y_px as f64 / scale + height;
        let geom = Cuboid::new(Vec2::new(width as f32, height as f32));
        let mut rb = RigidBody::new_dynamic(geom, 1.0f32, 0.3, 0.6);
        rb.append_translation(&Vec2::new(x as f32, y as f32));
        rb.append_rotation(&Vec1::new(0.01));
        let body = Rc::new(RefCell::new(rb));
        world.add_body(body.clone());
        let ctx = (*ctx).rgb(0.0, 0.0, 0.0);
        return WordBox { ctx: ctx, word: vec, body: body };
    }

    pub fn draw(&self, back_end: &mut Gl, scale: f64) {
        // Update
        let body = self.body.borrow();
        let delta: Iso2<f32> = One::one();
        let transform = body.transform_ref() * delta;
        let pos = na::translation(&transform);
        let rot = na::rotation(&transform);
        let geom = body.geom();
        let cuboid = geom.deref().downcast_ref::<geom::Cuboid>().unwrap();
        let dim = cuboid.half_extents();
        let ctx = self.ctx
            .trans(pos.x as f64 * scale, pos.y as f64 * scale)
            .rot_deg(rot.x.to_degrees() as f64);
        // Draw
        let mut x = (-dim.x as f64 * scale) as i32;
        let mut y = (dim.y as f64 * scale) as i32;
        for letter in self.word.iter() {
            let ctx = ctx.trans((x + letter.left) as f64, (y - letter.top) as f64);
            ctx
                .image(&letter.texture)
                .draw(back_end);
            x += (letter.advance.x >> 6) as i32;
            y += (letter.advance.y >> 6) as i32;
        }
    }
}
