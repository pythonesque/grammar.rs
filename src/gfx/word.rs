use ft = freetype;
use graphics::{
    AddImage,
    AddColor,
    Context,
    Draw,
    ImageColorContext,
    RelativeTransform2d,
};
use opengl_graphics::{
    Gl,
    Texture,
};
use std::cell::UnsafeCell;
use std::collections::hashmap::HashMap;
use std::kinds::marker;

struct Letter {
    texture: Texture,
    left: i32,
    top: i32,
    advance: ft::Vector,
}

pub struct WordContext<'dict> {
    face: ft::Face,
    letters: UnsafeCell<HashMap<char, Letter>>,
    marker: marker::NoShare
}

impl<'dict> WordContext<'dict> {
    pub fn make<'dict>() -> WordContext<'dict> {
        let freetype = ft::Library::init().unwrap();
        static font: &'static str = "/usr/share/fonts/TTF/DejaVuSans.ttf";
        let face = freetype.new_face(font, 0).unwrap();
        face.set_pixel_sizes(0, 48).unwrap();
        WordContext {
            face: face,
            letters: UnsafeCell::new(HashMap::new()),
            marker: marker::NoShare,
        }
    }

    fn letter(&self, ch: char) -> &Letter {
        let ref face = self.face;
        let letter;
        {
            // There can be no data races since WordContext is not Share.  We do not access the
            // hash table mutably outside of this block, and we do not access it immutably within
            // this block.
            letter = unsafe { (*(self.letters.get())).find_or_insert_with(ch, |&ch: &char| {
                face.load_char(ch as u64, ft::face::Render).unwrap();
                let g = face.glyph();
                let t = Texture::from_memory_alpha(
                    g.bitmap().buffer(),
                    g.bitmap().width() as u32,
                    g.bitmap().rows() as u32).unwrap();
                Letter { texture: t, left: g.bitmap_left(), top: g.bitmap_top(), advance: g.advance() }
            }) };
        }
        let &ref letter = letter;
        letter
    }
}

pub struct WordBox<'tree> {
    ctx: Vec<ImageColorContext<'tree, Texture>>,
}

impl<'tree> WordBox<'tree> {
    pub fn make<'dict, 'tree>(ctx: &Context, word_ctx: &'tree WordContext<'dict>, word: &'dict str) -> WordBox<'tree> {
        let mut x = 0;
        let mut y = 0;
        let ctx = (*ctx).rgb(0.0, 0.0, 0.0);
        let vec = word.chars().map( |ch| {
            let letter = word_ctx.letter(ch);
            let newctx = ctx.trans((x + letter.left) as f64, (y - letter.top) as f64);
            let newctx = newctx.image(&letter.texture);
            x += (letter.advance.x >> 6) as i32;
            y += (letter.advance.y >> 6) as i32;
            newctx
        }).collect();
        return WordBox { ctx: vec };
    }

    pub fn draw(&self, back_end: &mut Gl) {
        for ctx in self.ctx.iter() {
            ctx.draw(back_end);
        }
    }
}
