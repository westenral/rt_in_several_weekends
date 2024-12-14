mod camera;
mod color;
mod hit;
mod materials;
mod ray;
mod shapes;
mod vec3;

fn main() {
    // materials
    let mat_ground = materials::Lambertian {
        albedo: color::Color(0.8, 0.8, 0.),
    };
    let mat_center = materials::Lambertian {
        albedo: color::Color(0.1, 0.2, 0.5),
    };
    let mat_left = materials::Dialectric {
        refraction_index: 1.5,
    };
    let mat_right = materials::Metal {
        albedo: color::Color(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    // world setup
    let world = {
        let mut world = hit::HitList::default();

        // ground
        world.push(shapes::Sphere {
            center: vec3::Pos(0., -100.5, -1.),
            radius: 100.,
            mat: &mat_ground,
        });

        // center
        world.push(shapes::Sphere {
            center: vec3::Pos(0., 0., -1.2),
            radius: 0.5,
            mat: &mat_center,
        });

        // left
        world.push(shapes::Sphere {
            center: vec3::Pos(-1., 0., -1.),
            radius: 0.5,
            mat: &mat_left,
        });

        // right
        world.push(shapes::Sphere {
            center: vec3::Pos(1., 0., -1.),
            radius: 0.5,
            mat: &mat_right,
        });

        world
    };

    let cam = camera::CameraBuilder::default()
        .with_image_width(1920 / 4)
        .with_aspect_ratio(16.0 / 9.0)
        .with_samples_per_pixel(100)
        .with_max_bounces(50)
        .build();

    cam.render(&world);
}
