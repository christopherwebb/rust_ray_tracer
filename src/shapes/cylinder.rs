use std::f32;
use serde::{Deserialize, Serialize};

use crate::core::{
    Point3f,
    Vector3f,
    Normal3f,
    dot_vv,
    cross,
};
use crate::material::{
    Material,
    HitRecord
};
use crate::ray::Ray;
use crate::shapes::base::{solve_quadratic, Hitable};


#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Cylinder {
    pub centre : Point3f,
    pub radius: f32,
    pub phi_max: f32,
    pub z_min: f32,
    pub z_max: f32,
    pub material: Material,
}

impl Hitable for Cylinder {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let centre_vec = Vector3f::from(&self.centre);
        let oc = ray.origin() - &centre_vec;
        let oz_min = self.z_min - &self.centre.z;
        let oz_max = self.z_max - &self.centre.z;

        let a : f32 = &ray.b.x * &ray.b.x + &ray.b.y * &ray.b.y;
        let b : f32 = 2.0 * (&oc.x * &ray.b.x + &oc.y * &ray.b.y);
        let c : f32 = &oc.x * &oc.x + &oc.y * &oc.y - self.radius * self.radius;

        let (result, t0, t1) = solve_quadratic(a, b, c);
        if !result {
            return false;
        }

        if t0 > t_max || t1 <= 0.0 {return false;}

        // let mut t_hit : f32 = if t0 > t_min && t0 < t_max {
        //     t0
        // } else {
        //     t1
        // };

        let mut t_hit : f32 = if t0 <= 0.0 {
            t1
        } else {
            t0
        };

        // if t_hit > t_max {
        //     return false;
        // }

        if t_hit > t_min && t_hit < t_max {
        // {
            let mut hit = ray.point_at_parameter(t_hit);
            let mut orig_hit = hit - &centre_vec;

            // let mut hit_rad = (orig_hit.x * orig_hit.x + orig_hit.y * orig_hit.y).sqrt();
            // orig_hit.e[0] *= self.radius / hit_rad;
            // orig_hit.e[1] *= self.radius / hit_rad;

            // hit.e[0] = orig_hit.e[0] + self.centre.e[0];
            // hit.e[1] = orig_hit.e[1] + self.centre.e[1];

            let mut phi = orig_hit.y.atan2(orig_hit.x);
            
            if phi < 0.0 {
                phi += 2.0 * f32::consts::PI;
            }

            // if (orig_hit.z() < oz_min || orig_hit.z() > oz_max || phi > self.phi_max) {
            if orig_hit.z < oz_min || orig_hit.z > oz_max {
                if t_hit == t1 {
                    return false;
                }
                t_hit = t1;

                if t_hit > t_max {
                    return false;
                }

                hit = ray.point_at_parameter(t_hit);
                orig_hit = hit - &centre_vec;

                // hit_rad = (orig_hit.x * orig_hit.x + orig_hit.y * orig_hit.y).sqrt();
                // hit.e[0] *= self.radius / hit_rad;
                // hit.e[1] *= self.radius / hit_rad;

                // orig_hit.e[0] *= self.radius / hit_rad;
                // orig_hit.e[1] *= self.radius / hit_rad;

                // hit.e[0] = orig_hit.e[0] + self.centre.e[0];
                // hit.e[1] = orig_hit.e[1] + self.centre.e[1];

                // phi = orig_hit.y.atan2(orig_hit.x);
            
                // if phi < 0.0 {
                //     phi += 2.0 * f32::consts::PI;
                // }

                // if (orig_hit.z() < oz_min || orig_hit.z() > oz_max || phi > self.phi_max) {
                //     return false;
                // }

                if orig_hit.z < oz_min || orig_hit.z > oz_max {
                    return false;
                }
            }

            // let u = phi / self.phi_max;
            // // let v = (hit.z() - oz_min) / (oz_max - oz_min);
            // let v = (orig_hit.z - oz_min) / (oz_max - oz_min);

            let dpdu = Vector3f {
                x: -self.phi_max * orig_hit.y,
                y: self.phi_max * orig_hit.x,
                z: 0.0
            };
            let dpdv = Vector3f { x: 0.0, y: 0.0, z: oz_max - oz_min };

            // let d2Pduu = -self.phi_max * self.phi_max * Vector3f { x: orig_hit.x, y: orig_hit.y, z: 0.0 };
            // let d2Pduv = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            // let d2Pdvv = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

            // Compute coefficients for fundamental forms
            // let E = dot_vv(&dpdu, &dpdu);
            // let F = dot_vv(&dpdu, &dpdv);
            // let G = dot_vv(&dpdv, &dpdv);
            let N = &cross(&dpdu, &dpdv).unit_vector();
            // let e = dot_vv(&N, &d2Pduu);
            // let f = dot_vv(&N, &d2Pduv);
            // let g = dot_vv(&N, &d2Pdvv);

            // let invEGF2 = 1.0 / (E * G - F * F);
            // let dndu = &((f * F - e * G) * invEGF2 * dpdu +
            //                                        (e * F - f * E) * invEGF2 * dpdv).unit_vector();
            // let dndv = &((g * F - f * G) * invEGF2 * dpdu +
            //                                        (f * F - g * E) * invEGF2 * dpdv).unit_vector();

            rec.t = t_hit;
            rec.p = hit;
            rec.normal = Normal3f::from(N);
            rec.material = self.material;
            return true;
        }

        // let temp2 : f32 = (-b + (b * b - a * c).sqrt()) / a;
        // if temp2 > t_min && temp2 < t_max {
        //     rec.t = temp2;
        //     rec.p = ray.point_at_parameter(rec.t);
        //     rec.normal = (&rec.p - &self.centre) / self.radius;
        //     rec.material = self.material;
        //     return true;
        // }

        false
    }
}
