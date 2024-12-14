use super::*;
use crate::{color::Color, vec3::Vec3};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_info: &HitInfo) -> Option<(Ray, Color)> {
        let mut scatter_dir = Vec3::rand_unit_vec() + hit_info.normal;
        // catching problems
        if scatter_dir.near_zero() {
            scatter_dir = hit_info.normal;
        }

        let scattered = Ray {
            origin: hit_info.pos,
            dir: scatter_dir,
        };

        Some((scattered, self.albedo))
    }
}
