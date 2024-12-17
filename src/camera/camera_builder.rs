#![allow(unused)]

use super::Camera;
use crate::vec3::*;

// it's a little overkill...
#[derive(Default)]
pub struct CameraBuilder {
    aspect_ratio: Option<f64>,
    image_width: Option<u64>,
    samples_per_pixel: Option<u64>,
    max_bounces: Option<u64>,
    vfov: Option<f64>,
    lookat: Option<Pos>,
    lookfrom: Option<Pos>,
    vup: Option<Vec3>,
    defocus_angle: Option<f64>,
    focus_dist: Option<f64>,
}

macro_rules! with_param {
    ($param_name:ident, $param_type:ty, $fn_name:ident) => {
        pub fn $fn_name(self, $param_name: $param_type) -> Self {
            Self {
                $param_name: Some($param_name),
                ..self
            }
        }
    };
}

impl CameraBuilder {
    with_param!(aspect_ratio, f64, with_aspect_ratio);
    with_param!(image_width, u64, with_image_width);
    with_param!(samples_per_pixel, u64, with_samples_per_pixel);
    with_param!(max_bounces, u64, with_max_bounces);
    with_param!(vfov, f64, with_vfov);
    with_param!(lookat, Pos, with_lookat);
    with_param!(lookfrom, Pos, with_lookfrom);
    with_param!(vup, Vec3, with_vup);
    with_param!(defocus_angle, f64, with_defocus_angle);
    with_param!(focus_dist, f64, with_focus_dist);

    pub fn with_vfov_degrees(self, vfov: f64) -> Self {
        self.with_vfov(vfov.to_radians())
    }

    pub fn with_defocus_angle_degrees(self, defocus_angle: f64) -> Self {
        self.with_defocus_angle(defocus_angle.to_radians())
    }

    /// Preset resolution, aspect ratio, samples, and bounces for a final render.
    pub fn final_render() -> Self {
        Self::default()
            .with_image_width(1920)
            .with_aspect_ratio(16.0 / 9.0)
            .with_samples_per_pixel(1500)
            .with_max_bounces(200)
    }

    /// Preset resolution, aspect ratio, samples, and bounces for a debug render.
    pub fn debug_render() -> Self {
        Self::default()
            .with_image_width(1920 / 4)
            .with_aspect_ratio(16.0 / 9.0)
            .with_samples_per_pixel(100)
            .with_max_bounces(50)
    }

    pub fn build(self) -> Camera {
        Camera::new(
            self.aspect_ratio.unwrap_or(1.),
            self.image_width.unwrap_or(100),
            self.samples_per_pixel.unwrap_or(10),
            self.max_bounces.unwrap_or(10),
            self.vfov.unwrap_or(std::f64::consts::PI / 2.),
            self.lookat.unwrap_or(Vec3(0., 0., -1.)),
            self.lookfrom.unwrap_or(Vec3(0., 0., 0.)),
            self.vup.unwrap_or(Pos(0., 1., 0.)),
            self.defocus_angle.unwrap_or(0.),
            self.focus_dist.unwrap_or(10.),
        )
    }
}
