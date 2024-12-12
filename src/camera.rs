use crate::{color::*, hit::*, ray::*, shapes::*, vec3::*};

pub struct Camera {
    // pub aspect_ratio: f64,
    image_width: u64,
    image_height: u64,
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
    pub fn new(aspect_ratio: f64, image_width: u64) -> Self {
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
                let pixel_center =
                    self.pixel_00_pos + self.pixel_du * x as f64 + self.pixel_dv * y as f64;
                let ray_dir = pixel_center - self.center;

                let ray = Ray {
                    origin: self.center,
                    dir: ray_dir,
                };
                let ray_color = Camera::ray_color(&ray, world);

                ray_color.write_color();
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
}
