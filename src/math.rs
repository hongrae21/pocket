use sdl2::rect::Point;

use std::ops::Add;
use std::ops::Sub;
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

    fn dot(v1: Self, v2: Self) -> f32 {
        v1.x * v2.x + v1.y * v2.y
    }

    fn dist2(v1: Self, v2: Self) -> f32 {
        let d = v1 - v2;
        Self::dot(d, d)
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
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

pub fn collide_circle_and_ellipse(cir: &Circle, elp: &Ellipse) -> f32 {
    let mut mang = 0.0;
    let mut mdist2 = 1e9;
    for i in 0..360 {
        let ang = i as f32 * std::f32::consts::PI / 180.0;
        let pos = elp.pos + Vec2 {x: elp.a * f32::cos(ang), y: elp.b * f32::sin(ang)};
        let dist2 = Vec2::dist2(pos, cir.pos);
        if dist2 < mdist2 {
            mang = ang;
            mdist2 = dist2;
        }
    }
    if mdist2 <= cir.r * cir.r {
        return mang;
    } else {
        return -1.0;
    }
}
