#![feature(range_contains)]
#![feature(type_alias_enum_variants)]
use lazy_static::lazy_static;

mod dispatch_queue;
mod vec3;
mod ray;
mod shape;

use std::f64::MAX;
use self::{
    vec3::Vec3,
    ray::{Ray, Camera},
    shape::{Sphere, HitableList, Hitable},
    dispatch_queue::DispatchQueue
};
struct Env {
    camera: Camera,
    world: HitableList
}

lazy_static! {
    static ref Q: DispatchQueue = DispatchQueue::new("tracer", 6);
    static ref ENV: Env = {
        let camera = Camera::new(Vec3::new(-2.0, -1.0, -1.0),
                            Vec3::new(4.0, 0.0, 0.0),
                            Vec3::new(0.0, 2.0, 0.0),
                            Vec3::new(0.0, 0.0, 0.0));
        let s = Sphere::new(Vec3::new(0., 0., -1.), 0.5);
        let s2 = Sphere::new(Vec3::new(0., -100.5, -1.), 100.);

        let world  = HitableList { list: vec![s.into(), s2.into()] };

        Env {
            camera,
            world
        }

    };
}

static NX: usize = 1280;
static NY: usize = 720;

fn lerp(t: f64, start: Vec3, end: Vec3) -> Vec3 {
    (1. - t) * start + t * end
}

fn color(r: &Ray) -> Vec3 {

    if let Some(rec) = ENV.world.hit(r, 0.001, MAX) {
        let tgt = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * color(&Ray::new(rec.p,tgt - rec.p))
    }

    let unit_d = r.direction().get_unit_vector();
    let t = 0.5 * (unit_d.y() + 1.0);
    return lerp(t, Vec3::new(1., 1., 1.), Vec3::new(0.5, 0.7, 1.))

}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2. * Vec3::new(rand::random(), rand::random(), rand::random()) - Vec3::new(1., 1., 1.);

        if p.squared_len() < 1.0 {
            return p
        }
    }
}
fn get_color_from_pixel(pixel: usize) -> Vec3 {
    let ns = 100;

    let x = pixel % NX;
    let y = pixel / NX;
    
    let c = (0..ns)
        .map(|_| {
            let dx: f64 = rand::random();
            let dy: f64 = rand::random();

            let u = (x as f64 + dx) / NX as f64;
            let v = (y as f64 + dy) / NY as f64;

            let r = ENV.camera.get_ray(u, v);
            // let p = r.p(2.);

            color(&r)
        })
        .fold(Vec3::new(0., 0., 0.), |acc, x| acc + x);
    
    let c = (c / ns as f64).map(f64::sqrt)*259.99;

    c.map(f64::round)
}

fn p3() {
    let (tx, rx) = std::sync::mpsc::channel();

    println!("P3\n{} {}\n255\n", NX, NY);

    // let q = DispatchQueue::new("ray", 6);
    // let _q = &q;

    (0..NX*NY)
        .rev()
        .for_each(|pixel| {

            let tx = tx.clone();
            Q.dispatch(move || {
                let _ = tx.send((get_color_from_pixel(pixel), pixel));
            });
            
        });
    drop(tx);
    
    let mut res: Vec<_> = rx.iter().collect();
    res.sort_by_key(|k| k.1);

    res.iter()
        .rev()
        .for_each(|c| println!("{}", c.0));
}

fn main() {
    p3()
}