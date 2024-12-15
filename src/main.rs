mod camera;
mod color;
mod hit;
mod materials;
mod ray;
mod shapes;
mod vec3;

fn main() {
    let r = std::f64::consts::FRAC_PI_4.cos();

    // materials
    let mat_left = materials::Lambertian {
        albedo: color::Color(0., 0., 1.),
    };
    let mat_right = materials::Lambertian {
        albedo: color::Color(1., 0., 0.),
    };

    // world setup
    let world = {
        let mut world = hit::HitList::default();

        world.push(shapes::Sphere {
            center: vec3::Pos(-r, 0., -1.),
            radius: r,
            mat: &mat_left,
        });

        world.push(shapes::Sphere {
            center: vec3::Pos(r, 0., -1.),
            radius: r,
            mat: &mat_right,
        });

        world
    };

    let cam = camera::CameraBuilder::default()
        .with_image_width(1920 / 4)
        .with_aspect_ratio(16.0 / 9.0)
        .with_samples_per_pixel(100)
        .with_max_bounces(50)
        .with_vfov_degrees(90.)
        .build();

    cam.render(&world);
}
