#![allow(unused)]

mod color;
mod ray;
mod vec3;

use color::*;
use ray::*;
use vec3::*;

fn main() {
    // resolution of output image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    // minimum height of 1
    let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);

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

    println!("P3\n{image_width} {image_height}\n255");
    for y in 0..image_height {
        eprint!("\rLines remaining: {:>6}", image_height - y);
        for x in 0..image_width {
            let pixel_center = pixel_00_pos + pixel_du * x as f64 + pixel_dv * y as f64;
            let ray_dir = pixel_center - camera_position;

            let ray = Ray {
                origin: pixel_center,
                dir: ray_dir,
            };

            let ray_color = ray_color(&Ray {
                origin: pixel_center,
                dir: ray_dir,
            });

            ray_color.write_color();
        }
    }
    eprintln!();
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(ray, Pos(0., 0., -1.), 0.5) {
        return Color(1., 0., 0.);
    }

    let unit_ray = ray.dir.normalized();
    let scaled_y = (unit_ray.y() + 1.0) * 0.5;
    let c1 = Color(1., 1., 1.);
    let c2 = Color(0.5, 0.7, 1.0);
    c1 * (1.0 - scaled_y) + c2 * scaled_y
}

fn hit_sphere(ray: &Ray, center: Pos, radius: f64) -> bool {
    let oc = center - ray.origin;
    let a = ray.dir.dot(&ray.dir);
    let b = -2. * ray.dir.dot(&oc);
    let c = oc.dot(&oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.
}
