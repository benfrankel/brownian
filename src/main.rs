extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate cgmath;
extern crate rand;
extern crate clap;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use clap::{App as ClApp, Arg};

mod model;

use model::{ParticlePool, ParticlePoolGenerator};


pub struct App {
    gl: GlGraphics,
    pool: ParticlePool,
    pool_rand: ParticlePoolGenerator,
}

impl App {
    fn new(gl: GlGraphics, pool_rand: ParticlePoolGenerator, pool_size: usize) -> App {
        App {
            gl,
            pool: pool_rand.generate(pool_size),
            pool_rand,
        }
    }

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

            for gas in pool.particles.iter().skip(1) {
                let x = gas.position.x * disk_w / 2.0;
                let y = gas.position.y * disk_h / 2.0;
                let w = gas.radius * disk_w;
                let h = gas.radius * disk_h;
                ellipse(PARTICLE_COLOR, [x - w / 2.0, y - h / 2.0, w, h], transform, gl);
            }
            let dust = &pool.particles[0];
            let x = dust.position.x * disk_w / 2.0;
            let y = dust.position.y * disk_h / 2.0;
            let w = dust.radius * disk_w;
            let h = dust.radius * disk_h;
            ellipse(MAIN_PARTICLE_COLOR, [x - w / 2.0, y - h / 2.0, w, h], transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.pool.step(args.dt);
    }

    fn press(&mut self, button: &Button) {
        if let &Button::Keyboard(key) = button {
            match key {
                Key::Return => println!("Return?"),
                _ => (),
            }
        }
    }
}

fn main() {
    let matches = ClApp::new("Title Here")
        .version("0.1.0")
        .author("Ben Frankel <ben.frankel7@gmail.com>")
        .about("Brownian motion simulator")
        .arg(Arg::with_name("gas-count")
            .help("Number of gas particles")
            .long("count")
            .short("c")
            .takes_value(true)
            .default_value("1000"))
        .arg(Arg::with_name("gas-mass")
            .help("Average mass of the gas particles")
            .long("gas-mass")
            .short("m")
            .help("Mass of the dust particle")
            .takes_value(true)
            .default_value("1.0"))
        .arg(Arg::with_name("gas-size")
            .help("Average size of the gas particles")
            .long("gas-size")
            .short("s")
            .takes_value(true)
            .default_value("0.01"))
        .arg(Arg::with_name("gas-temperature")
            .help("Temperature of the gas")
            .long("gas-temp")
            .short("t")
            .takes_value(true)
            .default_value("1.0"))
        .arg(Arg::with_name("dust-mass")
            .help("Mass of the dust particle")
            .long("dust-mass")
            .short("M")
            .takes_value(true)
            .default_value("5.0"))
        .arg(Arg::with_name("dust-size")
            .help("Size of the dust particle")
            .long("dust-size")
            .short("S")
            .takes_value(true)
            .default_value("0.04"))
        .get_matches();

    let gas_count = matches.value_of("gas-count").unwrap().parse::<usize>().unwrap();
    let gas_mass = matches.value_of("gas-mass").unwrap().parse::<f64>().unwrap();
    let gas_radius = matches.value_of("gas-size").unwrap().parse::<f64>().unwrap();
    let gas_speed = matches.value_of("gas-temperature").unwrap().parse::<f64>().unwrap();
    let dust_mass = matches.value_of("dust-mass").unwrap().parse::<f64>().unwrap();
    let dust_radius = matches.value_of("dust-size").unwrap().parse::<f64>().unwrap();

    let opengl = OpenGL::V4_5;
    let mut window: Window = WindowSettings::new(
        "Spinning Square Demo",
        [512, 512],
    )
        .opengl(opengl)
        .srgb(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(
        GlGraphics::new(opengl),
        ParticlePoolGenerator::new(
            gas_mass, gas_mass / 8.0,
            gas_radius, gas_radius / 12.0,
            gas_speed, gas_speed / 8.0,
        ),
        gas_count,
    );

    app.pool.particles[0].mass = dust_mass;
    app.pool.particles[0].radius = dust_radius;
    app.pool.particles[0].position.x = 0.0;
    app.pool.particles[0].position.y = 0.0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(u) = e.press_args() {
            app.press(&u);
        }
    }
}
