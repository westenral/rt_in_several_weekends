use crate::{ray::*, vec3::*};
use std::ops::Range;

// information on ray intersection
pub struct HitInfo {
    /// the Position of the intersection
    pub pos: Pos,
    // the unit-length surface normal
    pub normal: Vec3,
    // the parameter to the ray
    pub t: f64,
    // whether the front or back face was hit
    pub front_face: bool,
}

// anything that can be hit by a ray
pub trait Hit {
    // calculates the hit info
    fn hit(&self, ray: &Ray, ray_t_interval: Range<f64>) -> Option<HitInfo>;
}

#[derive(Default)]
pub struct HitList {
    objects: Vec<Box<dyn Hit>>,
}

impl HitList {
    pub fn push(&mut self, object: Box<dyn Hit>) {
        self.objects.push(object)
    }
}

impl Hit for HitList {
    fn hit(&self, ray: &Ray, ray_t_interval: Range<f64>) -> Option<HitInfo> {
        self.objects
            .iter()
            .filter_map(|object| object.hit(ray, ray_t_interval.clone()))
            .min_by(|info1, info2| info1.t.total_cmp(&info2.t))
    }
}
