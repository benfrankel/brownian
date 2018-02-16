use std::ops::Add;


pub struct Vector {
    x: f64,
    y: f64,
}

impl Add for Vector {
    type output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


pub struct Particle {
    mass: f64,
    radius: f64,
    position: Vector,
    velocity: Vector,
}


impl Particle {
    fn new(mass: f64, radius: f64, x: f64, y: f64, speed: f64, angle: f64) -> Self {
        Particle {
            mass,
            radius,
            position: Vector { x, y },
            velocity: Vector { x: speed * cos(angle), y: speed * sin(angle) },
        }
    }

    fn step(&mut self, delta_time: f64) {
        self.x += delta_time * self.vx;
        self.y += delta_time * self.vy;
    }

    fn collides(&self, other: &Self) -> bool {
        // Particles are within each other's radii
        (self.x - other.x) ** 2 + (self.y - other.y) ** 2 < (self.r + other.r) ** 2

        // Particles are traveling towards each other
        && true
    }
}
