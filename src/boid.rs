use crate::wasm4::{line, SCREEN_SIZE};
use fastrand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn add(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
    pub fn sub(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
    }

    pub fn div(&mut self, n: f32) {
        self.x /= n;
        self.y /= n;
    }

    fn cartesian_to_polar(&self) -> (f32, f32) {
        let r = (self.x.powi(2) + self.y.powi(2)).sqrt();
        let theta = self.y.atan2(self.x);
        (r, theta)
    }

    pub fn set_magniute(&mut self, radius: f32) {
        let (_, theta) = self.cartesian_to_polar();
        self.x = radius * theta.cos();
        self.y = radius * theta.sin();
    }

    pub fn limit(&mut self, max_radius: f32) {
        let (radius, theta) = self.cartesian_to_polar();
        if radius > max_radius {
            self.x = max_radius * theta.cos();
            self.y = max_radius * theta.sin();
        }
    }

    pub fn distance(p1: &Point, p2: &Point) -> f32 {
        let dx = (p1.x - p2.x).abs();
        let dy = (p1.y - p2.y).abs();

        let screen_size: f32 = SCREEN_SIZE.into();
        let x_dist = dx.min(screen_size - dx);
        let y_dist = dy.min(screen_size - dy);

        f32::sqrt(x_dist * x_dist + y_dist * y_dist)
    }
}

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
        let screen_size:f32 = SCREEN_SIZE.into();
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

    pub fn update(&mut self, align: Point, cohesion: Point, seperation: Point) {
        self.position.add(self.velocity);

        let mut acceleration = Point { x: 0.0, y: 0.0 };
        acceleration.add(align);
        acceleration.add(cohesion);
        acceleration.add(seperation);
        acceleration.limit(Self::MAX_FORCE);

        self.velocity.add(acceleration);
        self.velocity.limit(Self::MAX_SPEED);
    }

    pub fn align(&self, boids: &[Self]) -> Point {
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

    pub fn cohesion(&self, boids: &[Self]) -> Point {
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

    pub fn seperation(&self, boids: &[Self]) -> Point {
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
