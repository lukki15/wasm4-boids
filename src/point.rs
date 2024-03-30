use crate::wasm4::SCREEN_SIZE;

#[derive(Clone, Copy, PartialEq)]
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
