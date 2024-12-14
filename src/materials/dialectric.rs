use super::Material;
use crate::{color::*, hit::HitInfo, ray::*};

pub struct Dialectric {
    pub refraction_index: f64,
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, hit_info: &HitInfo) -> Option<(Ray, Color)> {
        let attenuation = Color(1., 1., 1.);
        let ri = if hit_info.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = ray.dir.unit_vec();
        let refracted = unit_dir.refract(&hit_info.normal, ri);

        let scattered = Ray {
            origin: hit_info.pos,
            dir: refracted,
        };

        Some((scattered, attenuation))
    }
}
