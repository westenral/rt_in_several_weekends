use crate::{color::Color, hit::HitInfo, ray::Ray, vec3::*};

use super::Material;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_info: &HitInfo) -> Option<(Ray, Color)> {
        let scattered =
            ray.dir.reflect(&hit_info.normal).unit_vec() + Vec3::rand_unit_vec() * self.fuzz;
        match scattered.dot(&hit_info.normal) > 0. {
            true => Some((
                Ray {
                    origin: hit_info.pos,
                    dir: scattered,
                },
                self.albedo,
            )),
            false => None,
        }
    }
}
