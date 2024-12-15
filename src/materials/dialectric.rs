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

        let cos_theta = (-unit_dir).dot(&hit_info.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.;

        let dir = if cannot_refract || Dialectric::reflectance(cos_theta, ri) > fastrand::f64() {
            unit_dir.reflect(&hit_info.normal)
        } else {
            unit_dir.refract(&hit_info.normal, ri)
        };

        let scattered = Ray {
            origin: hit_info.pos,
            dir,
        };

        Some((scattered, attenuation))
    }
}

impl Dialectric {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // schlicky boi
        let r0 = (1. - refraction_index) / (1. + refraction_index);
        let r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}
