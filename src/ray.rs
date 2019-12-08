use crate::vector::Vec3;
use crate::core::Point3f;

pub struct Ray {
    pub a : Point3f,
    pub b : Vec3,
    pub time : f32,
}

impl Ray {
    pub fn origin(&self) -> Point3f { self.a.clone() }
    pub fn direction(&self) -> Vec3 { self.b.clone() }
    pub fn point_at_parameter(&self, point : f32) -> Point3f { &self.a + &(point * &self.b) }
}
