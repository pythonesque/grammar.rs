#![feature(unsafe_destructor)]

extern crate arena;
extern crate freetype;
extern crate glfw_game_window;
extern crate graphics;
extern crate nalgebra;
extern crate nphysics = "nphysics2df32";
extern crate ncollide = "ncollide2df32";
extern crate opengl_graphics;
extern crate piston;

use gfx::word::{WordBox, WordContext};
use glfw_game_window::GameWindowGLFW;
use gram::l0::{Noun, Verb, Pronoun, Determiner, S, ProNP, DetNomNP, Nominal, VP};
use graphics::{
    AddColor,
    AddSquareBorder,
    AddLine,
    Context,
    Draw,
};
use nalgebra::na::{Vec2, Translation};
use ncollide::geom::Plane;
use nphysics::object::RigidBody;
use nphysics::world::World;
use opengl_graphics::Gl;
use piston::{
    Game,
    GameIterator,
    GameWindowSettings,
    GameIteratorSettings,
    KeyPress,
    Render,
    Update,
    KeyPressArgs,
    RenderArgs,
    UpdateArgs
};
use piston::keyboard;
use std::cell::RefCell;
use std::rc::Rc;

mod gram;
mod gfx;

static meter: f64 = 32f64; /* 32 px / meter */
static width: f32 = 40.0; /* 40 m wide */
static height: f32 = 25.0; /* 25 m tall */
static width_px: u32 = (width as f64 * meter) as u32;
static height_px: u32 = (height as f64 * meter) as u32;

pub struct App<'a> {
    gl: Gl,       // OpenGL drawing backend.
    wc: &'a WordContext<'static, 'a>,
    world: World,
    wb: Vec<WordBox<'a>>,
}

impl<'a> App<'a> {
    fn render(&mut self, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);
        // Clear the screen.
        context.rgba(0.0,1.0,0.0,1.0).draw(&mut self.gl);
        // Draw the ground
        context
            .line(0.0, height_px as f64, width_px as f64, height_px as f64)
            .square_border_width(1.0)
            .rgba(0.0, 0.0, 0.0, 1.0)
            .draw(&mut self.gl);
        for ctx in self.wb.iter() {
            ctx.draw(&mut self.gl, meter);
        }
    }

    fn keypress(&mut self, args: &KeyPressArgs) {
        match args.key {
            keyboard::Space => {
                let context = &Context::abs(width_px as f64, height_px as f64);
                let wb = WordBox::make(
                    context,
                    self.wc,
                    &mut self.world,
                    meter,
                    "supercallafragallistic",
                );
                self.wb.push(wb);
            },
            _ => (),
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.world.step(args.dt as f32);
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
            size: [width_px, height_px],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    // Some settings for how the game should be run.
    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 60,
        max_frames_per_second: 60
    };

    // World
    let mut world = World::new();
    world.set_gravity(Vec2::new(0.0f32, 9.81));
    // Word boxes
    let wb = Vec::new();
    // Ground
    let mut rb = RigidBody::new_static(Plane::new(Vec2::new(0f32, -1.0)), 0.3, 0.6);
    rb.append_translation(&Vec2::new(0.0, height));
    let body = Rc::new(RefCell::new(rb));
    world.add_body(body.clone());
    // Wall
    let rb = RigidBody::new_static(Plane::new(Vec2::new(1.0, 0f32)), 0.3, 0.6);
    let body = Rc::new(RefCell::new(rb));
    world.add_body(body.clone());
    // Word context
    let mut wc = WordContext::make();
    // Create a new game and run it.
    let mut app = App {
        gl: Gl::new(),
        world: world,
        wc: &mut wc,
        wb: wb,
    };
    for mut e in GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(ref mut args) => { app.render(args) },
            Update(ref mut args) => { app.update(args) },
            KeyPress(ref mut args) => { app.keypress(args) },
            _ => (),
        }
    }
}
