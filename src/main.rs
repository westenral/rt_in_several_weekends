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

        world.push(Box::new(shapes::Sphere {
            center: vec3::Pos(0., -100.5, -1.),
            radius: 100.,
        }));

        world
    };

    let cam = camera::Camera::new(16.0 / 9.0, 400);
    cam.render(&world);
}
