use std::ops::Mul;
use crate::core::{
    Point3f,
    Vector3f,
    Matrix4x4f,
};


pub struct Ray {
    pub a : Point3f,
    pub b : Vector3f,
    pub time : f32,
}

impl Ray {
    pub fn origin(&self) -> Point3f { self.a.clone() }
    pub fn direction(&self) -> Vector3f { self.b.clone() }
    pub fn point_at_parameter(&self, point : f32) -> Point3f { &self.a + &(point * &self.b) }
}

impl Mul<Ray> for Matrix4x4f {
    type Output = Ray;

    fn mul(self, _rhs: Ray) -> Ray {
        Ray {
            a: self * _rhs.a,
            b: self * _rhs.b,
            time: _rhs.time,
        }
    }
}
