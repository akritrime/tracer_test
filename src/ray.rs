use super::vec3::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a, b }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn p(&self, t: f64) -> Vec3 {
        self.a + (self.b * t)
    }

}