use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Point {
        Point::new(value.0 as f32, value.1 as f32)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Point {
        Point::new(value.0 as f32, value.1 as f32)
    }
}

impl From<(f32, f32)> for Point {
    fn from(value: (f32, f32)) -> Point {
        Point::new(value.0, value.1)
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
    pub fn zero() -> Point {
        Point::new(0f32, 0f32)
    }
    pub fn between(&self, min: &Point, max: &Point) -> bool {
        min.x <= self.x && min.y <= self.y && max.x >= self.x && max.y >= self.y
    }

    pub fn to_slice(&self) -> [f32;2] {
        [self.x, self.y]
    }
}

impl ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Mul for Point {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl ops::Mul<f32> for Point {
    type Output = Point;
    fn mul(self, rhs: f32) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Div<f32> for Point {
    type Output = Point;
    fn div(self, rhs: f32) -> Point {
        Point::new(self.x / rhs, self.y / rhs)
    }
}