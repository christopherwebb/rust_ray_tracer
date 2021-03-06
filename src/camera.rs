use std::f32;
use rand::thread_rng;
use rand::Rng;

use serde::{Deserialize, Serialize};

use crate::core::{
    Point3f,
    Vector3f,
    cross,
};
use crate::ray::Ray;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Camera {
    origin : Point3f,
    lower_left_corner : Point3f,
    horizontal : Vector3f,
    vertical : Vector3f,
    u : Vector3f,
    v : Vector3f,
    w : Vector3f,
    lens_radius : f32,
    time_0 : f32,
    time_1 : f32,
}

impl Camera {
    pub fn create(
        look_from : Point3f,
        look_at : Point3f,
        up : Vector3f,
        fvov : f32,
        aspect : f32,
        aperature : f32,
        focus_dist : f32,
        time_0 : f32,
        time_1 : f32,
    ) -> Camera {
        let lens_radius = aperature / 2.0;

        let theta = fvov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (&look_from - &look_at).unit_vector();
        let u = cross(&up, &w).unit_vector();
        let v = cross(&w, &u);

        let half_width_u = half_width * &u * focus_dist;
        let half_height_v = half_height * &v * focus_dist;

        Camera {
            lower_left_corner: &look_from - &half_width_u - &half_height_v - focus_dist * w,
            origin: look_from,
            horizontal: 2.0 * half_width_u,
            vertical: 2.0 * half_height_v,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
            time_0: time_0,
			time_1: time_1,
        }
    }

    pub fn get_ray(self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Vector3f::rnd_in_unit_disc();
        let offset = self.u * rd.x + self.v * rd.y;

        let mut rng = thread_rng();
        let time = self.time_0 + (self.time_1 - self.time_0) * rng.gen::<f64>() as f32;

        Ray {
            a: &self.origin + &offset,
            b: &self.lower_left_corner
                + s * &self.horizontal
                + t * &self.vertical
                - &self.origin
                - offset,
            time: time,
        }
    }
}
