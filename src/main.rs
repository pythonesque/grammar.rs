extern crate graphics;
extern crate freetype;
extern crate piston;
extern crate glfw_game_window;
extern crate opengl_graphics;

use gram::l0::{Noun, Verb, Pronoun, Determiner, S, ProNP, DetNomNP, Nominal, VP};
use gfx::word::{WordBox, WordContext};

use glfw_game_window::GameWindowGLFW;
use opengl_graphics::Gl;

use piston::{
    Game,
    GameIterator,
    GameWindowSettings,
    GameIteratorSettings,
    Render,
    Update,
    RenderArgs,
    UpdateArgs
};

use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform2d,
};

mod gram;
mod gfx;

pub struct App<'a> {
    gl: Gl,       // OpenGL drawing backend.
    rotation: f64, // Rotation for the square.
    wc: &'a WordContext<'a>,
    wb: Option<WordBox<'a>>,
}

impl<'a> App<'a> {
    fn render(&mut self, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);
        // Clear the screen.
        context.rgba(0.0,1.0,0.0,1.0).draw(&mut self.gl);

        // Draw a box rotating around the middle of the screen.
        context
            .trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rot_rad(self.rotation)
            .rect(0.0, 0.0, 50.0, 50.0)
            .rgba(1.0, 0.0, 0.0,1.0)
            .trans(-25.0, -25.0)
            .draw(&mut self.gl);

        let ctx = match self.wb {
            Some(ref ctx) => ctx,
            None => {
                let wb = WordBox::make(&context.trans(0.0, 100.0), self.wc, "test");
                self.wb = Some(wb);
                self.wb.as_ref().unwrap()
            }
        };
        ctx.draw(&mut self.gl);
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let nouns = [Noun("morning"), Noun("flight")];
    let s = S {
        np: ProNP(Pronoun("I")),
        vp: VP {
            verb: Verb("prefer"),
            np: Some(DetNomNP(Determiner("a"), Nominal::make(nouns))),
            pp: None,
        },
    };
    println!("{:e}", s);
    println!("{}", s);
    // Create a GLFW window.
    let mut window = GameWindowGLFW::new(
        GameWindowSettings {
            title: "Hello Piston".to_string(),
            size: [800, 800],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    // Some settings for how the game should be run.
    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 60,
        max_frames_per_second: 60
    };

    // Create a new game and run it.
    let mut wc = WordContext::make();
    let mut app = App { gl: Gl::new(), rotation: 0.0, wc: &mut wc, wb: None };
    for mut e in GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(ref mut args) => { app.render(args); },
            Update(ref mut args) => { app.update(args); }
            _ => (),
        }
    }
}
