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

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3, origin: Vec3) -> Self {
        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical)
    }
}