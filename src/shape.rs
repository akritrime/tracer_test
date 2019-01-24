use std::convert::From;

use super::{
    vec3::Vec3,
    ray::Ray
};

pub trait Hitable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

// #[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3
}
pub enum Shape {
    _Sp(Sphere)
}

pub struct HitableList {
    pub list: Vec<Shape>
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

// impl HitRecord {
//     fn new(t: f64, p: Vec3, normal: Vec3) -> Self {
//         Self { t, p, normal }
//     }
// }

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere {
            center,
            radius
        }
    }

    // pub fn color(&self, r: &Ray) -> Vec3 {
    //     let h = self.hit(r, 0.0..1.0);
    //     if h > 0. {
    //         let n = (r.p(h) - Vec3::new(0.0, 0.0, -1.0)).get_unit_vector();
    //         return 0.5 * Vec3::new(n.x() + 1., n.y() + 1., n.z() + 1.)
    //     }

    //     let unit_v = r.direction().get_unit_vector();
    //     let t = 0.5*(unit_v.y() + 1.0);

    //     (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    // }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc =  r.origin() - self.center;
        let a  = r.direction().dot(r.direction());
        let b  = oc.dot(r.direction());
        let c  = oc.dot(oc) - self.radius.powi(2);

        let discriminant = b*b - a*c;

        let f = |t| {
            let p      = r.p(t);
            let normal = (p - self.center) / self.radius;
            Some(HitRecord {t, p, normal})
        };

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < tmax && temp > tmin {
                return f(temp)
            }

            let temp = (-b + discriminant.sqrt()) / a;
            if temp < tmax && temp > tmin {
                return f(temp)
            }
        }

        None
    }
}

impl Hitable for Shape {

    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        match *self {
            Self::_Sp(ref s) => s.hit(r, tmin, tmax)
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut hit_anything   = None;
        let mut closest_so_far = tmax;

        for hitable in &self.list {
            if let Some(tr) = hitable.hit(r, tmin, closest_so_far) {
                closest_so_far = tr.t;
                hit_anything   = Some(tr);
            }
        }

        hit_anything
    }
}

impl From<Sphere> for Shape {
    fn from(s: Sphere) -> Self {
        Shape::_Sp(s)
    }
}