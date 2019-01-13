use std::ops::{Add, Sub, Mul, Div, self};
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3([e0, e1, e2])
    }

    pub fn map(&self, f: fn(f64) -> f64) -> Vec3 {
        let e = self.0;

        Vec3([f(e[0]), f(e[1]), f(e[2])])
    }

    pub fn len(&self) -> f64 {
        self.squared_len().sqrt()
    }

    pub fn squared_len(&self) -> f64 {
        let f = |x| self[x] * self[x];
        f(0) + f(1) + f(2)

    }

    pub fn dot(&self, other: Vec3) -> f64 {
        (*self*other).iter().sum()
    }

    pub fn iter(&self) -> Iter<f64> {
        self.0.iter()
    }

    pub fn get_unit_vector(&self) -> Vec3 {
        let k = self[0] * self[0] + self[1] * self[1] + self[2] * self[2];
        let k = 1.0 / k.sqrt();

        self.clone() * k
    }

    pub fn x(&self) -> f64 {
        self.0[0] 
    }

    pub fn y(&self) -> f64 {
        self.0[1] 
    }

    pub fn z(&self) -> f64 {
        self.0[2] 
    }

    pub fn r(&self) -> f64 {
        self.0[0]
    }

    pub fn g(&self) -> f64 {
        self.0[1]
    }

    pub fn b(&self) -> f64 {
        self.0[2] 
    }

}

fn f<'a>(e: &'a[f64], e1: &'a[f64], ops: fn(f64, f64) -> f64) -> impl Fn(usize) -> f64 + 'a {
    move |x| ops(e[x], e1[x])
}

fn f1<'a>(e: &'a[f64], n: f64, ops: fn(f64, f64) -> f64) -> impl Fn(usize) -> f64 + 'a {
    move |x| ops(e[x], n)
}

macro_rules! impl_ops {
    ($t:tt, $n: tt) => {
        impl $t for Vec3 {
            type Output = Vec3;
            
            fn $n(self, other: Vec3) -> Vec3 {
                let e = self.0;
                let e1 = other.0;

                let f = f(&e, &e1, f64::$n);

                Vec3([f(0), f(1), f(2)])
            }
        }

        impl $t<f64> for Vec3 {
            type Output = Vec3;
            
            fn $n(self, other: f64) -> Vec3 {
                let e = self.0;

                let f = f1(&e, other, f64::$n);

                Vec3([f(0), f(1), f(2)])
            }
        }

        impl $t<Vec3> for f64 {
            type Output = Vec3;
            
            fn $n(self, other: Vec3) -> Vec3 {
                let e = other.0;

                let f = f1(&e, self, f64::$n);

                Vec3([f(0), f(1), f(2)])
            }
        }
    };
}

macro_rules! impl_ops_assign {
    ($t:tt, $n:tt, $n1:tt) => {
        // use std::ops::$n;
        impl ops::$t for Vec3 {

            fn $n1(&mut self, other: Vec3) {
                let e = self.0;
                let e1 = other.0;

                let f = f(&e, &e1, f64::$n);

                *self =  Vec3([f(0), f(1), f(2)])
            }
        }

        impl ops::$t<f64> for Vec3 {
            fn $n1(&mut self, other: f64) {
                let e = self.0;

                let f = f1(&e, other, f64::$n);

                *self = Vec3([f(0), f(1), f(2)])
            }
        }
    };
}

impl_ops!(Add, add);
impl_ops!(Sub, sub);
impl_ops!(Mul, mul);
impl_ops!(Div, div);

impl_ops_assign!(AddAssign, add, add_assign);
impl_ops_assign!(SubAssign, sub, sub_assign);
impl_ops_assign!(MulAssign, mul, mul_assign);
impl_ops_assign!(DivAssign, div, div_assign);

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        &self.0[idx]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut f64 {
        &mut self.0[idx]
    }
}

