use crate::boid::Boid;
use fastrand::Rng;

const BOID_COUNT: usize = 64;

pub struct Game {
    boids: [Boid; BOID_COUNT],
}

impl Game {
    pub fn new() -> Self {
        let rng = Rng::with_seed(254);
        Self {
            boids: core::array::from_fn(|i| Boid::new(i, &rng)),
        }
    }

    pub fn update(&mut self) {
        let aligns: [crate::boid::Point; BOID_COUNT] =
            core::array::from_fn(|i| self.boids[i].align(&self.boids));
        let cohesion: [crate::boid::Point; BOID_COUNT] =
            core::array::from_fn(|i| self.boids[i].cohesion(&self.boids));
        let seperations: [crate::boid::Point; BOID_COUNT] =
            core::array::from_fn(|i| self.boids[i].seperation(&self.boids));

        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.edges();
            boid.update(aligns[i], cohesion[i], seperations[i]);
            boid.draw();
        }
    }
}
