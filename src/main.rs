#![feature(range_contains)]
#![feature(type_alias_enum_variants)]

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

fn lerp(t: f64, start: Vec3, end: Vec3) -> Vec3 {
    (1. - t) * start + t * end
}

fn color(r: &Ray, world: &HitableList) -> Vec3 {

    if let Some(rec) = world.hit(r, 0., MAX) {
        return 0.5 * (rec.normal + 1.)
    }

    let unit_d = r.direction().get_unit_vector();
    let t = 0.5 * (unit_d.y() + 1.0);
    return lerp(t, Vec3::new(1., 1., 1.), Vec3::new(0.5, 7., 1.))

}

// fn color(r: &Ray) -> Vec3 {
//     let t = hit_sphere(Vec3::new(0., 0., -1.), 0.5, r);
//     if t > 0. {
//         let normal = (r.p(t) - Vec3::new(0., 0., -1.)).get_unit_vector();
//         return 0.5 * (normal + 1.)
//     }
//     let u_d = r.direction().get_unit_vector();
//     let t   = 0.5 * (u_d.y() + 1.);
    
//     // lerp(t, Vec3::new(0.9, 0.2, 0.), Vec3::new(0.8, 1., 0.2))
//     Vec3::new(0., 0., 0.)
// }

// fn hit_sphere(c: Vec3, rd: f64, r: &Ray) -> f64 {
//     let oc = r.origin() - c;
//     let a = r.direction().dot(r.direction());
//     let b = 2. * r.direction().dot(oc);
//     let c = oc.dot(oc) - (rd * rd);
//     let discriminant = b*b - 4.*a*c;

//     if discriminant > 0. {
//         (-b - discriminant.sqrt()) / (2. * a)
//     } else {
//         -1.
//     }
// }
fn get_color_from_pixel(pixel: usize) -> Vec3 {
    let nx = 960;
    let ny = 480;
    let ns = 100;


    let camera = Camera::new(Vec3::new(-2.0, -1.0, -1.0),
                            Vec3::new(4.0, 0.0, 0.0),
                            Vec3::new(0.0, 2.0, 0.0),
                            Vec3::new(0.0, 0.0, 0.0));
    let s = Sphere::new(Vec3::new(0., 0., -1.), 0.5);
    let s2 = Sphere::new(Vec3::new(0., -100.5, -1.), 100.);

    let world  = HitableList { list: vec![s.into(), s2.into()] };

    let x = pixel % nx;
    let y = pixel / nx;
    
    let c = (0..ns)
        .map(|_| {
            let dx: f64 = rand::random();
            let dy: f64 = rand::random();

            let u = (x as f64 + dx) / nx as f64;
            let v = (y as f64 + dy) / ny as f64;

            let r = camera.get_ray(u, v);
            // let p = r.p(2.);

            color(&r, &world)
        })
        .fold(Vec3::new(0., 0., 0.), |acc, x| acc + x);
    
    let c = (c / ns as f64)*259.99;

    c.map(f64::round)
}

fn p3() {
    let nx = 960;
    let ny = 480;
    // let ns = 100;

    let (tx, rx) = std::sync::mpsc::channel();

    // let camera = Arc::new(camera);

    println!("P3\n{} {}\n255\n", nx, ny);

    let q = DispatchQueue::new("ray", 4);

    // let rng = rand::thread_rng();

    (0..ny*nx)
        .rev()
        .for_each(|pixel| {

            let tx = tx.clone();
            // let f = |x, y, nx, ny, camera: Arc<Camera>| ;

            q.dispatch(move || {
    
                let _ = tx.send(get_color_from_pixel(pixel));
            });
            
            // println!("{}", c.map(f64::round));
        });
    
    let _v: Vec<_> = rx.iter()
        .map(|c| println!("{}", c))
        .collect();
    // for c in rx.iter() {
        
    // }



    // for j in (0..ny).rev() {
    //     for i in 0..nx {
    //         let c = (0..ns)
    //             .map(|_| f(i, j))
    //             .fold(Vec3::new(0., 0., 0.), |acc, x| acc + x);
    //         let c = (c / ns as f64)*259.99;
            
    //         println!("{}", c.map(f64::round));
    //     }
    // }
}

fn main() {
    p3()
}