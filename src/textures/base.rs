use crate::core::{Point3f, Normal3f, Colour};

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Point3f) -> Colour;
}
