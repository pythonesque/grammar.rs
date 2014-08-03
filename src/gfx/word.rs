use ft = freetype;
use graphics::{
    AddRectangle,
    AddImage,
    AddColor,
    Context,
    Draw,
    ImageColorContext,
    RectangleContext,
    RelativeTransform2d,
};
use opengl_graphics::{
    Gl,
    Texture,
};

pub struct WordContext {
    freetype: ft::Library,
    face: ft::Face,
    texture: Texture,
}

impl WordContext {
    pub fn make() -> WordContext {
        let freetype = ft::Library::init().unwrap();
        static font: &'static str = "/usr/share/fonts/TTF/DejaVuSans.ttf";
        let face = freetype.new_face(font, 0).unwrap();
        face.set_pixel_sizes(0, 48).unwrap();
        let ch = 'a';/*word.chars().next().unwrap();*/
        face.load_char(ch as u64, ft::face::Render).unwrap();
        let texture;
        {
            let g = face.glyph();
            texture = Texture::from_memory_alpha(g.bitmap().buffer(), g.bitmap().width() as u32, g.bitmap().rows() as u32).unwrap();
        }
        WordContext { freetype: freetype, face: face, texture: texture }
    }
}

pub struct WordBox<'tree> {
    ctx: ImageColorContext<'tree, Texture>,
    //word: &'dict str,
    //texture: Box<Texture>,
}

impl<'tree> WordBox<'tree> {
    pub fn make<'dict, 'tree>(ctx: &Context, word_ctx: &'tree WordContext, word: &'dict str) -> WordBox<'tree> {
        let /*mut*/ x = 0;
        let /*mut*/ y = 0;
        /*for ch in word.chars() 
        {*/
            let g = word_ctx.face.glyph();
            let newctx = (*ctx).trans((x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64);
            let newctx = newctx.image(&word_ctx.texture);
            let newctx = newctx.rgb(0.0, 0.0, 0.0);
            return WordBox { ctx: newctx };
            /*x += (g.advance().x >> 6) as i32;
            y += (g.advance().y >> 6) as i32;*/
        /*}*/
    }

    pub fn draw(&self, back_end: &mut Gl) {
        self.ctx.draw(back_end);
    }
}
