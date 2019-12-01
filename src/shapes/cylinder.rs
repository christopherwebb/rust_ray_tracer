use std::f32;
use serde::{Deserialize, Serialize};

use crate::material::{
    Material,
    HitRecord
};
use crate::ray::Ray;
use crate::shapes::base::{solve_quadratic, Hitable};
use crate::vector::{
    Vec3,
    dot,
    unit_vector,
    cross,
};


#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Cylinder {
    pub centre : Vec3,
    pub radius: f32,
    pub phi_max: f32,
    pub zMin: f32,
    pub zMax: f32,
    pub material: Material,
}

impl Hitable for Cylinder {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc : Vec3 = ray.origin() - &self.centre;
        let oz_min = self.zMin - &self.centre.z();
        let oz_max = self.zMax - &self.centre.z();

        let a : f32 = &ray.b.x() * &ray.b.x() + &ray.b.y() * &ray.b.y();
        let b : f32 = 2.0 * (&oc.x() * &ray.b.x() + &oc.y() * &ray.b.y());
        let c : f32 = &oc.x() * &oc.x() + &oc.y() * &oc.y() - self.radius * self.radius;

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
            let mut orig_hit = hit - self.centre;

            let mut hit_rad = (orig_hit.x() * orig_hit.x() + orig_hit.y() * orig_hit.y()).sqrt();
            // orig_hit.e[0] *= self.radius / hit_rad;
            // orig_hit.e[1] *= self.radius / hit_rad;

            // hit.e[0] = orig_hit.e[0] + self.centre.e[0];
            // hit.e[1] = orig_hit.e[1] + self.centre.e[1];

            let mut phi = orig_hit.y().atan2(orig_hit.x());
            
            if phi < 0.0 {
                phi += 2.0 * f32::consts::PI;
            }

            // if (orig_hit.z() < oz_min || orig_hit.z() > oz_max || phi > self.phi_max) {
            if orig_hit.z() < oz_min || orig_hit.z() > oz_max {
                if t_hit == t1 {
                    return false;
                }
                t_hit = t1;

                if t_hit > t_max {
                    return false;
                }

                hit = ray.point_at_parameter(t_hit);
                orig_hit = hit - self.centre;

                hit_rad = (orig_hit.x() * orig_hit.x() + orig_hit.y() * orig_hit.y()).sqrt();
                // hit.e[0] *= self.radius / hit_rad;
                // hit.e[1] *= self.radius / hit_rad;

                // orig_hit.e[0] *= self.radius / hit_rad;
                // orig_hit.e[1] *= self.radius / hit_rad;

                // hit.e[0] = orig_hit.e[0] + self.centre.e[0];
                // hit.e[1] = orig_hit.e[1] + self.centre.e[1];

                phi = orig_hit.y().atan2(orig_hit.x());
            
                if phi < 0.0 {
                    phi += 2.0 * f32::consts::PI;
                }

                // if (orig_hit.z() < oz_min || orig_hit.z() > oz_max || phi > self.phi_max) {
                //     return false;
                // }

                if orig_hit.z() < oz_min || orig_hit.z() > oz_max {
                    return false;
                }
            }

            let u = phi / self.phi_max;
            // let v = (hit.z() - oz_min) / (oz_max - oz_min);
            let v = (orig_hit.z() - oz_min) / (oz_max - oz_min);

            // Vector3f dpdu(-self.phi_max * hit.y(), self.phi_max * hit.x(), 0);
            // Vector3f dpdv(0, 0, zMax - zMin);

            // let dpdu = Vec3 { e: [-self.phi_max * hit.y(), self.phi_max * hit.x(), 0.0]};
            let dpdu = Vec3 { e: [-self.phi_max * orig_hit.y(), self.phi_max * orig_hit.x(), 0.0]};
            let dpdv = Vec3 { e: [0.0, 0.0, oz_max - oz_min]};

            // let d2Pduu = -self.phi_max * self.phi_max * Vec3 { e: [hit.x(), hit.y(), 0.0]};
            let d2Pduu = -self.phi_max * self.phi_max * Vec3 { e: [orig_hit.x(), orig_hit.y(), 0.0]};
            let d2Pduv = Vec3 { e: [0.0, 0.0, 0.0]};
            let d2Pdvv = Vec3 { e: [0.0, 0.0, 0.0]};

            // Compute coefficients for fundamental forms
            let E = dot(&dpdu, &dpdu);
            let F = dot(&dpdu, &dpdv);
            let G = dot(&dpdv, &dpdv);
            let N = unit_vector(&cross(&dpdu, &dpdv));
            let e = dot(&N, &d2Pduu);
            let f = dot(&N, &d2Pduv);
            let g = dot(&N, &d2Pdvv);

            let invEGF2 = 1.0 / (E * G - F * F);
            let dndu = unit_vector(&((f * F - e * G) * invEGF2 * dpdu + 
                                                   (e * F - f * E) * invEGF2 * dpdv));
            let dndv = unit_vector(&((g * F - f * G) * invEGF2 * dpdu + 
                                                   (f * F - g * E) * invEGF2 * dpdv));

            rec.t = t_hit;
            rec.p = hit;
            // rec.normal = (&rec.p - &self.centre) / self.radius;
            rec.normal = N;
            // normal is Normalize(Cross(dpdu, dpdv))?
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
