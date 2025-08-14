use std::time::Duration;

use crate::math::{Vec2, Circle};


pub struct Ball {
    pub obj: Circle,
    pub dir: Vec2,
    pub sped: f32
}

// TODO: ball size should be adjusted. so some refactoring be needed.
impl Ball {
    pub fn update(&mut self, dt: Duration) {
        self.obj.pos += self.dir.clone() * self.sped.clone() * dt.as_secs_f32();
    }
}
