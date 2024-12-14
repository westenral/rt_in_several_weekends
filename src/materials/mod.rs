pub mod lambertian;
pub mod metal;

pub use lambertian::*;
pub use metal::*;

use crate::{color::Color, hit::HitInfo, ray::Ray};

pub trait Material {
    /// Given an in-ray and hit info, returns the scattered ray and attenuated color
    fn scatter(&self, ray: &Ray, hit_info: &HitInfo) -> Option<(Ray, Color)>;
}
