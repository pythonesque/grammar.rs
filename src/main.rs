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
    GameWindow,
    GameWindowSettings,
    GameIteratorSettings,
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

pub struct App {
    gl: Gl,       // OpenGL drawing backend.
    rotation: f64, // Rotation for the square.
    wc: WordContext,
}

impl <W: GameWindow> Game<W> for App {
    fn render(&mut self, _: &mut W, args: &RenderArgs) {
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

//        let rect = context.rect(0.0, 1.0, 0.0, 1.0);
        let ctx = WordBox::make(&context.trans(0.0, 100.0), &self.wc, "test");
        ctx.draw(&mut self.gl);
    }

    fn update(&mut self, _: &mut W, args: &UpdateArgs) {
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
    let mut app = App { gl: Gl::new(), rotation: 0.0, wc: WordContext::make() };
    app.run(&mut window, &game_iter_settings);
}
