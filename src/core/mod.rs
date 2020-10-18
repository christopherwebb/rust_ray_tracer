mod point;
pub use self::point::{
    Point2f,
    Point2i,
    Point3f,
    Point3i,
};

mod vector;
pub use self::vector::{
    Vector2f,
    Vector2i,
    Vector3f,
    Vector3i,
    Vector3T,
    cross,
    // dot,
};

mod normal;
pub use self::normal::{
    Normal2f,
    Normal2i,
    Normal3f,
    Normal3i,
    Normal3T,
};

mod matrix;
pub use self::matrix::{
    Matrix4x4i,
    Matrix4x4f,
    indentity,
};

mod transforms;
pub use self::transforms::{
    Transform,
    gen_translate,
    gen_scale,
    gen_rotate_x,
    gen_rotate_y,
    gen_rotate_z,
    gen_rotate,
};

mod colour;
pub use self::colour::Colour;

use std::ops::{
    Add,
    Mul,
};

pub fn dot_vv<T>(l: &Vector3T<T>, r: &Vector3T<T>) -> T
    where T: Mul + Copy + Add,
          T: Mul<Output=T>,
          T: Add<Output=T>,

{
    l.x * r.x + l.y * r.y + l.z * r.z
}

pub fn dot_vn<T>(l: &Vector3T<T>, r: &Normal3T<T>) -> T
    where T: Mul + Copy + Add,
          T: Mul<Output=T>,
          T: Add<Output=T>,

{
    l.x * r.x + l.y * r.y + l.z * r.z
}

pub fn dot_nv<T>(l: &Normal3T<T>, r: &Vector3T<T>) -> T
    where T: Mul + Copy + Add,
          T: Mul<Output=T>,
          T: Add<Output=T>,

{
    l.x * r.x + l.y * r.y + l.z * r.z
}

pub fn dot_nn<T>(l: &Normal3T<T>, r: &Normal3T<T>) -> T
    where T: Mul + Copy + Add,
          T: Mul<Output=T>,
          T: Add<Output=T>,

{
    l.x * r.x + l.y * r.y + l.z * r.z
}

// TODO: Should a reflection fireback a Normal? Or whatever type v is?
pub fn reflect(v: &Vector3f, n: &Normal3f) -> Vector3f {
    v - Vector3f::from(&(2.0 * dot_vn(v, n) * n))
}

pub fn refract(v : &Vector3f, n : &Normal3f, ni_over_nt : f32) -> (bool, Option<Vector3f>) {
    let uv = v.unit_vector();
    let dt = dot_vn(&uv, n);
    let discriminant : f32 = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        (true, Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt()))
    } else {
        (false, None)
    }
}
