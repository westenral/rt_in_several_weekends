use crate::{ray::*, vec3::*};

// information on ray intersection
pub struct HitInfo {
    /// the Position of the intersection
    pub pos: Pos,
    // the unit-length surface normal
    pub normal: Vec3,
    // the parameter to the ray
    pub t: f64,
}

// anything that can be hit by a ray
pub trait Hit {
    // calculates the hit info
    fn hit(&self, ray: &Ray, ray_t_interval: std::ops::Range<f64>) -> Option<HitInfo>;
}
