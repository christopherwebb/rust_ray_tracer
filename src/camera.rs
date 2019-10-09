use std::f32;
use rand::thread_rng;
use rand::Rng;

use crate::vector::{
    Vec3,
    cross,
    unit_vector,
    rnd_in_unit_disc,
};
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Camera {
    origin : Vec3,
    lower_left_corner : Vec3,
    horizontal : Vec3,
    vertical : Vec3,
    u : Vec3,
    v : Vec3,
    w : Vec3,
    lens_radius : f32,
    time_0 : f32,
    time_1 : f32,
}

impl Camera {
    pub fn create(
        look_from : Vec3,
        look_at : Vec3,
        up : Vec3,
        fvov : f32,
        aspect : f32,
        aperature : f32,
        focus_dist : f32,
        time_0 : f32,
        time_1 : f32,
    ) -> Camera {
        let u : Vec3;
        let v : Vec3;
        let w : Vec3;
        let lens_radius = aperature / 2.0;

        let theta = fvov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        w = unit_vector(&(&look_from - &look_at));
        u = unit_vector(&cross(&up, &w));
        v = cross(&w, &u);

        let half_width_u = half_width * &u * focus_dist;
        let half_height_v = half_height * &v * focus_dist;

        Camera {
            lower_left_corner: &look_from - &half_width_u - &half_height_v - focus_dist * w,
            origin: look_from,
            horizontal: 2.0 * half_width_u,
            vertical: 2.0 * half_height_v,
            // horizontal: Vec3 {e:[1.0, 1.0, 1.0]},
            // vertical: Vec3 {e:[1.0, 1.0, 1.0]},
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
            time_0: time_0,
			time_1: time_1,
        }
    }
    pub fn get_ray(self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * rnd_in_unit_disc();
        // let offset = self.u * rd.x() + self.v * rd.y();
        let offset = Vec3 {e:[0.0, 0.0, 0.0]};

        let mut rng = thread_rng();
        let time = self.time_0 + (self.time_1 - self.time_0) * rng.gen::<f64>() as f32;

        Ray {
            a: self.origin.clone() + offset,
            b: &self.lower_left_corner + &(s * &self.horizontal) + t * &self.vertical - &self.origin - offset,
            time: time,
        }
    }
}
