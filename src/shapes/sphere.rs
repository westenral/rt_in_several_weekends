use crate::{hit::*, materials::Material, ray::*, vec3::*};
use std::ops::Range;

pub struct Sphere<'a> {
    pub center: Pos,
    // radius should be positive
    pub radius: f64,

    // material
    // could be seperated - geometry and material seperate
    pub mat: &'a dyn Material,
}

impl Hit for Sphere<'_> {
    fn hit(&self, ray: &Ray, ray_t_interval: Range<f64>) -> Option<HitInfo> {
        // quadratic formula
        // simplified when b = -2h

        let oc = self.center - ray.origin;

        let a = ray.dir.length_squared();
        let h = ray.dir.dot(&oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }

        // calculate nearest intersection
        let sqrt_d = discriminant.sqrt();
        let mut t = (h - sqrt_d) / a;
        if !ray_t_interval.contains(&t) {
            t = (h + sqrt_d) / a;
            if !ray_t_interval.contains(&t) {
                return None;
            }
        }

        let pos = ray.at(t);
        let out_normal = (pos - self.center) / self.radius;
        let front_face = out_normal.dot(&ray.dir) < 0.;
        let normal = if front_face { out_normal } else { -out_normal };
        Some(HitInfo {
            pos,
            normal,
            t,
            front_face,
            mat: self.mat,
        })
    }
}

// #[test]
// fn sphere_hit_test() {
//     let ray = Ray {
//         origin: Pos(0., 0., 0.),
//         dir: Vec3(0., 0., -1.),
//     };
//     let sphere = Sphere {
//         center: Pos(0., 0., -1.),
//         radius: 0.5,
//     };

//     let hit_info = sphere.hit(&ray, 0.0..f64::INFINITY).unwrap();
//     assert!(hit_info.t == 0.5);
// }
