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
    let material_center = materials::Lambertian {
        albedo: color::Color(0.1, 0.2, 0.5),
    };
    let material_left = materials::Dialectric {
        refraction_index: 1.50,
    };
    let material_bubble = materials::Dialectric {
        refraction_index: 1.00 / 1.50,
    };
    let material_right = materials::Metal {
        albedo: color::Color(0.8, 0.6, 0.2),
        fuzz: 1.,
    };

    // world setup
    let world = {
        let mut world = hit::HitList::default();

        world.push(shapes::Sphere::new(
            vec3::Pos(0., -100.5, -1.),
            100.,
            &material_ground,
        ));

        world.push(shapes::Sphere::new(
            vec3::Pos(0., 0., -1.2),
            0.5,
            &material_center,
        ));

        world.push(shapes::Sphere::new(
            vec3::Pos(-1., 0., -1.),
            0.5,
            &material_left,
        ));

        world.push(shapes::Sphere::new(
            vec3::Pos(-1., 0., -1.),
            0.4,
            &material_bubble,
        ));

        world.push(shapes::Sphere::new(
            vec3::Pos(1., 0., -1.),
            0.5,
            &material_right,
        ));

        world
    };

    let cam = camera::CameraBuilder::debug_render()
        .with_vfov_degrees(20.)
        .with_lookfrom(vec3::Pos(-2., 2., 1.))
        .with_lookat(vec3::Pos(0., 0., -1.))
        .with_vup(vec3::Vec3(0., 1., 0.))
        .with_defocus_angle_degrees(10.)
        .with_focus_dist(3.4)
        .build();

    cam.render(&world);
}
