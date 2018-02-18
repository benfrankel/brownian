use cgmath::{Vector2, MetricSpace, InnerSpace};
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range, Normal};


pub struct Particle {
    pub mass: f64,
    pub radius: f64,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    has_collided: bool,
}


impl Particle {
    fn new(mass: f64, radius: f64, x: f64, y: f64, speed: f64, angle: f64) -> Self {
        Particle {
            mass,
            radius,
            position: Vector2 { x, y },
            velocity: Vector2 { x: speed * angle.cos(), y: speed * angle.sin() },
            has_collided: false,
        }
    }

    fn step(&mut self, delta_time: f64) {
        self.position += delta_time * self.velocity;
        self.collide_unit_circle();
    }

    /// Returns true if collision occurs between the two particles, and false otherwise.
    /// Adjusts position and velocity due to elastic collision.
    fn collide_particle(&mut self, other: &mut Self) -> bool {
        // Ensure particles are in contact
        if self.position.distance2(other.position) > (self.radius + other.radius).powi(2) {
            return false;
        }

        // Ensure particles are traveling towards each other
        let dx = other.position - self.position;
        if dx.dot(self.velocity) <= 0.0 && dx.dot(other.velocity) >= 0.0 {
            return false;
        }

        // Backtrack overlap
        let overlap = (self.radius + other.radius - self.position.distance(other.position)) / 2.0;
        let normal = dx;
        let normal = normal.normalize_to(overlap);
        self.position -= normal;
        other.position += normal;

        // Update velocities (elastic collision)
        let dv = other.velocity - self.velocity;
        let mass = self.mass + other.mass;
        let temp = self.velocity + 2.0 * other.mass / mass * dv.dot(dx) / dx.magnitude2() * dx;
        other.velocity = other.velocity - 2.0 * self.mass / mass * dv.dot(dx) / dx.magnitude2() * dx;
        self.velocity = temp;

        self.has_collided = true;
        true
    }

    /// Returns true if collision occurs with the unit circle boundary, and false otherwise.
    /// Adjusts position and velocity due to collision.
    fn collide_unit_circle(&mut self) -> bool {
        if self.position.magnitude2() < (1.0 - self.radius).powi(2) {
            return false;
        }

        self.position = self.position.normalize_to(1.0 - self.radius);
        self.velocity -= 2.0 * self.velocity.dot(self.position) * self.position;

        self.has_collided = true;
        true
    }
}


pub struct ParticlePoolGenerator {
    mass_rng: Box<Normal>,
    radius_rng: Box<Normal>,
    speed_rng: Box<Normal>,
    angle_rng: Box<Range<f64>>,
    position_rng: Box<Range<f64>>,
}

impl ParticlePoolGenerator {
    pub fn new(mass_avg: f64, mass_stdev: f64,
               radius_avg: f64, radius_stdev: f64,
               speed_avg: f64, speed_stdev: f64) -> ParticlePoolGenerator {
        ParticlePoolGenerator {
            mass_rng: Box::new(Normal::new(mass_avg, mass_stdev)),
            radius_rng: Box::new(Normal::new(radius_avg, radius_stdev)),
            speed_rng: Box::new(Normal::new(speed_avg, speed_stdev)),
            angle_rng: Box::new(Range::new(0.0, 6.28)),
            position_rng: Box::new(Range::new(-1.0, 1.0)),
        }
    }

    pub fn generate(&self, count: usize) -> ParticlePool {
        let mut rng = thread_rng();
        let mut pool = ParticlePool::new(count);

        for _ in 0..count {
            // Generate position in unit circle
            let mut x = self.position_rng.ind_sample(&mut rng);
            let mut y = self.position_rng.ind_sample(&mut rng);
            while (x * x + y * y) >= 1.0 {
                x = self.position_rng.ind_sample(&mut rng);
                y = self.position_rng.ind_sample(&mut rng);
            }

            pool.particles.push(Particle::new(
                self.mass_rng.ind_sample(&mut rng),
                self.radius_rng.ind_sample(&mut rng),
                x, y,
                self.speed_rng.ind_sample(&mut rng),
                self.angle_rng.ind_sample(&mut rng),
            ));
        }

        pool
    }
}


pub struct ParticlePool {
    pub particles: Vec<Particle>,
    speed: f64,
}


impl ParticlePool {
    pub fn new(count: usize) -> ParticlePool {
        ParticlePool {
            particles: Vec::with_capacity(count),
            speed: 1.0,
        }
    }

    pub fn step(&mut self, delta_time: f64) {
        for particle in self.particles.iter_mut() {
            particle.step(delta_time * self.speed);
        }

        for i in 1..self.particles.len() {
            let (left, right) = self.particles.split_at_mut(i);
            let p = &mut right[0];
            if p.has_collided {
                continue;
            }

            for q in left.iter_mut().filter(|q| !q.has_collided) {
                p.collide_particle(q);
            }
        }

        for particle in self.particles.iter_mut() {
            particle.has_collided = false;
        }
    }
}
