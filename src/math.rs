pub struct Ellipse {
    // (x - x1)^2 / a^2 + (y - y1)^2 / b^2 = 1
    pub x: f32,
    pub y: f32,
    pub a: f32,
    pub b: f32,
}

pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32
}
