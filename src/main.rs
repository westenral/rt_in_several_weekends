mod camera;
mod color;
mod hit;
mod materials;
mod ray;
mod shapes;
mod vec3;

fn main() {
    // materials
    let material_ground = materials::Lambertian {
        albedo: color::Color(0.8, 0.8, 0.0),
    };
    let mat1 = materials::Dialectric {
        refraction_index: 1.5,
    };
    let mat2 = materials::Lambertian {
        albedo: color::Color(0.4, 0.2, 0.1),
    };
    let mat3 = materials::Metal {
        albedo: color::Color(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    let material_list = {
        let mut material_list: Vec<Box<dyn materials::Material + Sync>> = vec![];

        for _ in -11..11 {
            for _ in -11..11 {
                let choose_mat = fastrand::f64();

                match choose_mat {
                    0.0..0.8 => {
                        let albedo = color::Color::random() * color::Color::random();
                        material_list.push(Box::new(materials::Lambertian { albedo }));
                    }
                    ..0.95 => {
                        let albedo = color::Color::random_range(0.5, 1.);
                        let fuzz = fastrand::f64() * 0.5;
                        material_list.push(Box::new(materials::Metal { albedo, fuzz }));
                    }
                    _ => {
                        material_list.push(Box::new(materials::Dialectric {
                            refraction_index: 1.5,
                        }));
                    }
                }
            }
        }

        material_list
    };

    // world setup
    let world = {
        let mut world = hit::HitList::default();

        world.push(shapes::Sphere::new(
            vec3::Pos(0., -1000., 0.),
            1000.,
            &material_ground,
        ));
        world.push(shapes::Sphere::new(vec3::Pos(0., 1., 0.), 1., &mat1));
        world.push(shapes::Sphere::new(vec3::Pos(-4., 1., 0.), 1., &mat2));
        world.push(shapes::Sphere::new(vec3::Pos(4., 1., 0.), 1., &mat3));

        let mut i: usize = 0;
        for a in -11..11 {
            for b in -11..11 {
                let radius = 0.2;
                let center = vec3::Pos(
                    a as f64 + 0.9 * fastrand::f64(),
                    radius,
                    b as f64 + 0.9 * fastrand::f64(),
                );

                world.push(shapes::Sphere::new(center, radius, &*material_list[i]));

                i += 1;
            }
        }

        world
    };

    let cam = camera::CameraBuilder::final_render()
        .with_vfov_degrees(20.)
        .with_lookfrom(vec3::Pos(13., 2., 3.))
        .with_lookat(vec3::Pos(0., 0., 0.))
        .with_vup(vec3::Vec3(0., 1., 0.))
        .with_defocus_angle_degrees(0.6)
        .with_focus_dist(10.0)
        .build();

    // cam.render(&world);
    cam.render_parallel(&world);
}
