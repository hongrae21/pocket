use std::time::Duration;

use crate::math::Vec2;
use crate::math::Circle;


pub struct Ball {
    pub obj: Circle,
    pub dir: Vec2,
    pub sped: f32
}

impl Ball {
    pub fn update(&mut self, dt: Duration) {
        self.obj.pos += self.dir.clone() * self.sped.clone() * dt.as_secs_f32();
    }
}
