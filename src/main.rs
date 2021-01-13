extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const SCREEN_WIDTH: u32 = 475;
const SCREEN_HEIGHT: u32 = 475;

struct Ship {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
    angle: i32,
    radius: i32
}

impl Ship {
    pub fn update(&self) {

    }
    pub fn draw(&self) {

    }
    pub fn fire_bullet(&self) {

    }
}

pub struct App {
    gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            line_from_to(RED, 3.0, [5.0, 5.0], [475.0, 475.0], c.transform, gl);
            line_from_to(WHITE, 3.0, [475.0, 5.0], [5.0, 475.0], c.transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("rusteroids", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut app = App {
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
