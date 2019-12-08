use crate::material::HitRecord;

use crate::ray::Ray;

pub trait Hitable {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut HitRecord
    ) -> bool;
}

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> (bool, f32, f32) {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return (false, 0.0, 0.0);
    }

    let t0 = -0.5 * (b + discriminant.sqrt()) / a;
    let t1 = -0.5 * (b - discriminant.sqrt()) / a;

    if t0 > t1 {
        return (true, t1, t0);
    } else {
        return (true, t0, t1);
    }
}