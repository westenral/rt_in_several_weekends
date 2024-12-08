use crate::color::*;
use crate::vec3::*;

pub struct Ray {
    pub origin: Pos,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Pos {
        self.origin + self.dir * t
    }
}
