#![feature(range_contains)]
#![feature(type_alias_enum_variants)]


mod dispatch_queue;
mod vec3;
mod ray;
mod shape;

use std::f64::MAX;
use self::{
    vec3::Vec3,
    ray::Ray,
    shape::{Sphere, HitableList, Hitable},
};

fn lerp(t: f64) -> Vec3 {
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn color(r: &Ray, world: &HitableList) -> Vec3 {

    if let Some(rec) = world.hit(r, 0.0..MAX) {
        return 0.5 * (rec.normal + 1.)
    }

    let unit_d = r.direction().get_unit_vector();
    let t = 0.5 * (unit_d.y() + 1.0);
    return lerp(t)

}

fn p3() {
    let nx = 200;
    let ny = 100;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let s = Sphere::new(Vec3::new(0., 0., -1.), 0.5);
    let s2 = Sphere::new(Vec3::new(0., -100.5, -1.), 100.);

    let world = HitableList { list: vec![s.into(), s2.into()] };

    println!("P3\n{} {}\n255\n", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;

            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            // let p = r.p(2.);

            let c = (color(&r, &world) * 255.99)
                .map(|n| n.round());
            
            println!("{} {} {}", c[0], c[1], c[2]);
        }
    }
}

fn main() {
    p3()
}