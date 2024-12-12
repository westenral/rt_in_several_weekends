mod camera;
mod color;
mod hit;
mod ray;
mod shapes;
mod vec3;

fn main() {
    // world setup
    let world = {
        let mut world = hit::HitList::default();

        world.push(Box::new(shapes::Sphere {
            center: vec3::Pos(0., 0., -1.),
            radius: 0.5,
        }));

        // world.push(Box::new(shapes::Sphere {
        //     center: vec3::Pos(-2., -0.0, -2.),
        //     radius: 0.2,
        // }));

        // world.push(Box::new(shapes::Sphere {
        //     center: vec3::Pos(1., 1., -1.),
        //     radius: 0.3,
        // }));

        // world.push(Box::new(shapes::Sphere {
        //     center: vec3::Pos(1., 2., -5.),
        //     radius: 3.,
        // }));

        world.push(Box::new(shapes::Sphere {
            center: vec3::Pos(0., -100.5, -1.),
            radius: 100.,
        }));

        world
    };

    let cam = camera::CameraBuilder::default()
        .with_image_width(400)
        .with_aspect_ratio(16.0 / 9.0)
        .with_samples_per_pixel(50)
        .with_max_bounces(50)
        .build();

    cam.render(&world);
}
