use crate::vector::Vec3;

pub struct Ray {
    pub a : Vec3,
    pub b : Vec3,
    pub time : f32,
}

impl Ray {
    pub fn origin(&self) -> Vec3 { self.a.clone() }
    pub fn direction(&self) -> Vec3 { self.b.clone() }
    pub fn point_at_parameter(&self, point : f32) -> Vec3 { &self.a + &(point * &self.b) }
}
