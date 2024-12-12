use crate::{color::*, hit::*, ray::*, vec3::*};

// it's a little overkill...
#[derive(Default)]
pub struct CameraBuilder {
    aspect_ratio: Option<f64>,
    image_width: Option<u64>,
    samples_per_pixel: Option<u64>,
}

impl CameraBuilder {
    pub fn with_aspect_ratio(self, aspect_ratio: f64) -> Self {
        CameraBuilder {
            aspect_ratio: Some(aspect_ratio),
            ..self
        }
    }

    pub fn with_image_width(self, image_width: u64) -> Self {
        CameraBuilder {
            image_width: Some(image_width),
            ..self
        }
    }

    pub fn with_samples_per_pixel(self, samples_per_pixel: u64) -> Self {
        CameraBuilder {
            samples_per_pixel: Some(samples_per_pixel),
            ..self
        }
    }

    pub fn build(self) -> Camera {
        Camera::new(
            self.aspect_ratio.unwrap_or(1.),
            self.image_width.unwrap_or(100),
            self.samples_per_pixel.unwrap_or(10),
        )
    }
}

pub struct Camera {
    // pub aspect_ratio: f64,
    image_width: u64,
    image_height: u64,
    samples_per_pixel: u64,
    pixel_sample_scale: f64,
    // focal_length: f64,
    // viewport_upper_left: Pos,
    pixel_00_pos: Pos,
    // viewport_u: Vec3,
    // viewport_v: Vec3,
    pixel_du: Vec3,
    pixel_dv: Vec3,
    center: Pos,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u64, samples_per_pixel: u64) -> Self {
        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;

        // minimum height of 1
        let image_height = ((image_width as f64 / aspect_ratio) as u64).max(1);

        // camera info
        let camera_position = Pos(0., 0., 0.);
        let focal_length = 1.0;

        // camera viewport info
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        // vectors along viewport top and left edges
        let viewport_u = Vec3(viewport_width, 0., 0.);
        let viewport_v = Vec3(0., -viewport_height, 0.);

        // horizontal and vertical vec between in-world pixel centers
        let pixel_du = viewport_u / image_width as f64;
        let pixel_dv = viewport_v / image_height as f64;

        // position of upper left pixel
        let viewport_upper_left =
            camera_position + Vec3(0., 0., -focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel_00_pos = viewport_upper_left + pixel_du / 2. + pixel_dv / 2.;

        Self {
            // aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            pixel_sample_scale,
            // focal_length,
            // viewport_upper_left,
            pixel_00_pos,
            // viewport_u,
            // viewport_v,
            pixel_du,
            pixel_dv,
            center: camera_position,
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
                    color += Camera::ray_color(&ray, world);
                }

                (color * self.pixel_sample_scale).write_color();
            }
        }

        eprintln!(
            "\rFinished rendering in {:.4} seconds                           ",
            start_time.elapsed().as_millis() as f64 / 1000.0
        );
    }

    fn ray_color(ray: &Ray, world: &impl Hit) -> Color {
        if let Some(hit_info) = world.hit(ray, 0.0..f64::INFINITY) {
            return (hit_info.normal + 1.0) / 2.0;
        }

        let unit_ray = ray.dir.unit_vec();
        let scaled_y = (unit_ray.y() + 1.0) * 0.5;
        let c1 = Color(1., 1., 1.);
        let c2 = Color(0.5, 0.7, 1.0);
        c1 * (1.0 - scaled_y) + c2 * scaled_y
    }

    fn get_ray(&self, i: u64, j: u64) -> Ray {
        let offset = sample_square();
        Ray {
            origin: self.center,
            dir: self.pixel_00_pos
                + self.pixel_du * (i as f64 + offset.0)
                + self.pixel_dv * (j as f64 + offset.1)
                - self.center,
        }
    }
}

// return [-0.5, -0.5] - [0.5, 0.5]
fn sample_square() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let x: f64 = rand::Rng::gen_range(&mut rng, -0.5..0.5);
    let y: f64 = rand::Rng::gen_range(&mut rng, -0.5..0.5);
    (x, y)
}
