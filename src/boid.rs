use crate::point::Point;
use crate::wasm4::{line, SCREEN_SIZE};
use fastrand::Rng;

pub struct Boid {
    id: usize,
    position: Point,
    velocity: Point,
}

impl Boid {
    const MAX_FORCE: f32 = 0.2;
    const MAX_SPEED: f32 = 0.8;

    fn random_direction(rng: &Rng, max_radius: f32) -> Point {
        let radius = rng.f32() * max_radius + 0.5;
        let theta = rng.f32() * std::f32::consts::PI * 2.0;
        Point {
            x: radius * theta.cos(),
            y: radius * theta.sin(),
        }
    }

    pub fn new(id: usize, rng: &Rng) -> Self {
        Self {
            id,
            position: Point {
                x: rng.f32() * f32::from(SCREEN_SIZE),
                y: rng.f32() * f32::from(SCREEN_SIZE),
            },
            velocity: Self::random_direction(rng, Self::MAX_SPEED),
        }
    }

    pub fn draw(&self) {
        let x = self.position.x as i32;
        let y = self.position.y as i32;
        line(x, y, x, y);
    }

    pub fn edges(&mut self) {
        let screen_size: f32 = SCREEN_SIZE.into();
        if self.position.x < 0.0 {
            self.position.x += screen_size;
        } else if self.position.x > f32::from(SCREEN_SIZE - 1) {
            self.position.x -= screen_size;
        }

        if self.position.y < 0.0 {
            self.position.y += screen_size;
        } else if self.position.y > f32::from(SCREEN_SIZE - 1) {
            self.position.y -= screen_size;
        }
    }

    pub fn update(&mut self, acceleration: &Acceleration) {
        self.position.add(self.velocity);

        let mut acceleration_sum = acceleration.sum();
        acceleration_sum.limit(Self::MAX_FORCE);

        self.velocity.add(acceleration_sum);
        self.velocity.limit(Self::MAX_SPEED);
    }

    fn align(&self, boids: &[Self]) -> Point {
        const PERCEPTION_RADIUS: f32 = 5.0;
        let mut avg = Point { x: 0.0, y: 0.0 };
        let mut total: u16 = 0;

        for boid in boids {
            if self.id == boid.id {
                continue;
            }

            let d = Point::distance(&self.position, &boid.position);
            if d < PERCEPTION_RADIUS {
                avg.add(boid.velocity);
                total += 1;
            }
        }

        if total > 0 {
            avg.div(total.into());
            avg.set_magniute(Self::MAX_SPEED);
            avg.sub(self.velocity);
            avg.limit(Self::MAX_FORCE);
        }
        avg
    }

    fn cohesion(&self, boids: &[Self]) -> Point {
        const PERCEPTION_RADIUS: f32 = 5.0;
        let mut avg = Point { x: 0.0, y: 0.0 };
        let mut total: u16 = 0;

        for boid in boids {
            if self.id == boid.id {
                continue;
            }

            let d = Point::distance(&self.position, &boid.position);
            if d < PERCEPTION_RADIUS {
                avg.add(boid.position);
                total += 1;
            }
        }

        if total > 0 {
            avg.div(total.into());
            avg.sub(self.position);
            avg.set_magniute(Self::MAX_SPEED);
            avg.sub(self.velocity);
            avg.limit(Self::MAX_FORCE);
        }

        avg
    }

    fn seperation(&self, boids: &[Self]) -> Point {
        const PERCEPTION_RADIUS: f32 = 5.0;
        let mut avg = Point { x: 0.0, y: 0.0 };
        let mut total: u16 = 0;

        for boid in boids {
            let d = Point::distance(&self.position, &boid.position);
            if self.id != boid.id && d < PERCEPTION_RADIUS {
                let mut diff = Point {
                    x: self.position.x - boid.position.x,
                    y: self.position.y - boid.position.y,
                };
                diff.div(d * d);
                avg.add(diff);
                total += 1;
            }
        }

        if total > 0 {
            avg.div(total.into());
            avg.set_magniute(Self::MAX_SPEED);
            avg.sub(self.velocity);
            avg.limit(Self::MAX_FORCE);
        }

        avg
    }
}

pub struct Acceleration {
    align: crate::boid::Point,
    cohesion: crate::boid::Point,
    seperation: crate::boid::Point,
}

impl Acceleration {
    pub fn calc(boid: &Boid, boids: &[Boid]) -> Self {
        Acceleration {
            align: boid.align(boids),
            cohesion: boid.cohesion(boids),
            seperation: boid.seperation(boids),
        }
    }

    fn sum(&self) -> Point {
        let mut acceleration = Point { x: 0.0, y: 0.0 };
        acceleration.add(self.align);
        acceleration.add(self.cohesion);
        acceleration.add(self.seperation);
        acceleration
    }
}
