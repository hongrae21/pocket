use sdl2::rect::Point;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn to_point(&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub struct Circle {
    pub pos: Vec2,
    pub r: f32
}

pub struct Ellipse {
    // (x - x1)^2 / a^2 + (y - y1)^2 / b^2 = 1
    pub pos: Vec2,
    pub a: f32,
    pub b: f32,
}
