extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate cgmath;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

mod model;

use model::{ParticlePool, ParticlePoolGenerator};


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend
    pool: ParticlePool,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG_COLOR: [f32; 4] = [0.04, 0.04, 0.08, 1.0];
        const DISK_COLOR: [f32; 4] = [0.12, 0.12, 0.16, 1.0];
        const PARTICLE_COLOR: [f32; 4] = [0.24, 0.24, 0.31, 1.0];
        const MAIN_PARTICLE_COLOR: [f32; 4] = [0.39, 0.78, 0.39, 1.0];

        let disk_w = (args.width as f64) * 0.9;
        let disk_h = (args.height as f64) * 0.9;
        let disk_x = ((args.width as f64) - disk_w) / 2.0;
        let disk_y = ((args.height as f64) - disk_h) / 2.0;

        let pool = &self.pool;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BG_COLOR, gl);

            let transform = c.transform
                .trans(disk_x, disk_y)
                .trans(disk_w / 2.0, disk_h / 2.0);

            ellipse(DISK_COLOR, [disk_x, disk_y, disk_w, disk_h], c.transform, gl);

            for particle in pool.particles.iter().skip(1) {
                let x = particle.position.x * disk_w / 2.0;
                let y = particle.position.y * disk_h / 2.0;
                let w = particle.radius * disk_w;
                let h = particle.radius * disk_h;
                ellipse(PARTICLE_COLOR, [x - w / 2.0, y - h / 2.0, w, h], transform, gl);
            }
            let particle = &pool.particles[0];
            let x = particle.position.x * disk_w / 2.0;
            let y = particle.position.y * disk_h / 2.0;
            let w = particle.radius * disk_w;
            let h = particle.radius * disk_h;
            ellipse(MAIN_PARTICLE_COLOR, [x - w / 2.0, y - h / 2.0, w, h], transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.pool.step(args.dt);
    }
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new(
        "Spinning Square Demo",
        [200, 200],
    )
        .opengl(opengl)
        .srgb(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let pool_rand = ParticlePoolGenerator::new(
        1.0, 0.1,
        0.02, 0.0,
        1.0, 0.5,
    );

    let mut app = App {
        gl: GlGraphics::new(opengl),
        pool: pool_rand.generate(500),
    };

    // app.pool.particles[0].mass = 10.0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
