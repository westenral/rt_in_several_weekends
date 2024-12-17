mod camera_builder;
pub use camera_builder::*;

use crate::{color::*, hit::*, ray::*, vec3::*};

pub struct Camera {
    // pub aspect_ratio: f64,
    image_width: u64,
    image_height: u64,
    // vfov: f64,
    // focal_length: f64,
    // viewport_upper_left: Pos,
    pixel_00_pos: Pos,
    // viewport_u: Vec3,
    // viewport_v: Vec3,
    pixel_du: Vec3,
    pixel_dv: Vec3,
    center: Pos,

    samples_per_pixel: u64,
    pixel_sample_scale: f64,
    max_bounces: u64,

    defocus_angle: f64,
    defocus_u: Vec3,
    defocus_v: Vec3,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    /// Use CameraBuilder instead.
    pub fn new(
        aspect_ratio: f64,
        image_width: u64,
        samples_per_pixel: u64,
        max_bounces: u64,
        vfov: f64,
        lookat: Pos,
        lookfrom: Pos,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;

        // minimum height of 1
        let image_height = ((image_width as f64 / aspect_ratio) as u64).max(1);

        // camera info
        let camera_position = lookfrom;
        // let focal_length = (lookfrom - lookat).length();
        let theta = vfov;
        let h = (theta / 2.).tan();

        // camera viewport info
        let viewport_height = 2. * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // calculate orthonormal basis (u, v, w)
        let w = (lookfrom - lookat).unit_vec();
        let u = vup.cross(&w).unit_vec();
        let v = w.cross(&u);

        // vectors along viewport top and left edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // horizontal and vertical vec between in-world pixel centers
        let pixel_du = viewport_u / image_width as f64;
        let pixel_dv = viewport_v / image_height as f64;

        // position of upper left pixel
        let viewport_upper_left =
            camera_position - (focus_dist * w) - viewport_u / 2. - viewport_v / 2.;
        let pixel_00_pos = viewport_upper_left + pixel_du / 2. + pixel_dv / 2.;

        let defocus_radius = focus_dist * (defocus_angle / 2.).tan();
        let defocus_u = u * defocus_radius;
        let defocus_v = v * defocus_radius;

        Self {
            // aspect_ratio,
            image_width,
            image_height,
            // vfov,
            // focal_length,
            // viewport_upper_left,
            pixel_00_pos,
            // viewport_u,
            // viewport_v,
            pixel_du,
            pixel_dv,
            center: camera_position,

            samples_per_pixel,
            pixel_sample_scale,
            max_bounces,

            defocus_angle,
            defocus_u,
            defocus_v,
        }
    }

    // outputs to stdout rn...
    pub fn render(&self, world: &impl Hit) {
        let start_time = std::time::Instant::now();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for y in 0..self.image_height {
            eprint!("\rLines remaining: {:>6}", self.image_height - y);

            for x in 0..self.image_width {
                let mut color = Color(0., 0., 0.);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color += self.ray_color(&ray, world, 0);
                }

                (color * self.pixel_sample_scale).write_color();
            }
        }

        eprintln!(
            "\rFinished rendering in {:.4} seconds                           ",
            start_time.elapsed().as_millis() as f64 / 1000.0
        );
    }

    fn ray_color(&self, ray: &Ray, world: &impl Hit, bounces: u64) -> Color {
        if bounces > self.max_bounces {
            return Color(0., 0., 0.);
        }

        // object intersection
        if let Some(hit_info) = world.hit(ray, 0.001..f64::INFINITY) {
            // uniform distribution
            // let next_dir = Vec3::random_on_hemisphere(&hit_info.normal);
            // lambertian distribution
            // let next_dir = Vec3::rand_unit_vec() + hit_info.normal;
            // let next_ray = Ray {
            //     origin: hit_info.pos,
            //     dir: next_dir,
            // };
            // return 0.5 * self.ray_color(&next_ray, world, bounces + 1);

            if let Some((ray, attenuation)) = hit_info.mat.scatter(ray, &hit_info) {
                return attenuation * self.ray_color(&ray, world, bounces + 1);
            }
            return Color(0., 0., 0.);
        }

        // background color
        let unit_ray = ray.dir.unit_vec();
        let scaled_y = (unit_ray.y() + 1.0) * 0.5;
        let c1 = Color(1., 1., 1.);
        let c2 = Color(0.5, 0.7, 1.0);
        c1 * (1.0 - scaled_y) + c2 * scaled_y

        // Color(1., 1., 1.)
    }

    fn get_ray(&self, i: u64, j: u64) -> Ray {
        let origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let offset = sample_square();
        // let offset = offset.0 * self.pixel_du + offset.1 * self.pixel_dv;
        let viewport_target = self.pixel_00_pos
            + self.pixel_du * (offset.0 + i as f64)
            + self.pixel_dv * (offset.1 + j as f64);
        let dir = viewport_target - origin;

        Ray { origin, dir }
    }

    fn defocus_disk_sample(&self) -> Pos {
        let p = Vec3::rand_in_unit_disk();
        self.center + p.0 * self.defocus_u + p.1 * self.defocus_v
    }
}

// return [-0.5, -0.5] - [0.5, 0.5]
fn sample_square() -> (f64, f64) {
    (fastrand::f64() - 0.5, fastrand::f64() - 0.5)
}
