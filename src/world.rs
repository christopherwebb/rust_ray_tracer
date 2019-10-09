use std::f32;
use crate::material::{
    Material,
    HitRecord
};

use crate::ray::Ray;

use crate::vector::{
    Vec3,
    dot,
    unit_vector,
    cross,
};

pub trait Hitable {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub centre : Vec3,
    pub radius: f32,
    pub material: Material,
}

#[derive(Copy, Clone)]
pub struct MovingSphere {
    pub centre0 : Vec3,
    pub time0 : f32,

    pub centre1 : Vec3,
    pub time1 : f32,

    pub radius: f32,
    pub material: Material,
}

impl MovingSphere {
    fn centre(&self, time: f32) -> Vec3 {
        let time_fac = (time - self.time0) / (self.time1 - self.time0);
        self.centre0 + time_fac * (self.centre1 - self.centre0)
    }
}

#[derive(Copy, Clone)]
pub struct Cylinder {
    pub centre : Vec3,
    pub radius: f32,
    pub phi_max: f32,
    pub zMin: f32,
    pub zMax: f32,
    pub material: Material,
}

impl Hitable for Sphere {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc : Vec3 = ray.origin() - &self.centre;

        let a : f32 = dot(&ray.direction(), &ray.direction());
        let b : f32 = dot(&oc, &ray.direction());
        let c : f32 = dot(&oc, &oc) - self.radius * self.radius;

        let discriminant : f32 = b * b - a * c;

        if discriminant <= 0.0 {
            return false;
        }

        let temp : f32 = (-b - (b * b - a * c).sqrt()) / a;
        if temp > t_min && temp < t_max {
            rec.t = temp;
            rec.p = ray.point_at_parameter(rec.t);
            rec.normal = (&rec.p - &self.centre) / self.radius;
            rec.material = self.material;
            return true;
        }

        let temp2 : f32 = (-b + (b * b - a * c).sqrt()) / a;
        if temp2 > t_min && temp2 < t_max {
            rec.t = temp2;
            rec.p = ray.point_at_parameter(rec.t);
            rec.normal = (&rec.p - &self.centre) / self.radius;
            rec.material = self.material;
            return true;
        }

        false
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc : Vec3 = ray.origin() - &self.centre(ray.time);

        let a : f32 = dot(&ray.direction(), &ray.direction());
        let b : f32 = dot(&oc, &ray.direction());
        let c : f32 = dot(&oc, &oc) - self.radius * self.radius;

        let discriminant : f32 = b * b - a * c;

        if discriminant <= 0.0 {
            return false;
        }

        let temp : f32 = (-b - (b * b - a * c).sqrt()) / a;
        if temp > t_min && temp < t_max {
            rec.t = temp;
            rec.p = ray.point_at_parameter(rec.t);
            rec.normal = (&rec.p - &self.centre(ray.time)) / self.radius;
            rec.material = self.material;
            return true;
        }

        let temp2 : f32 = (-b + (b * b - a * c).sqrt()) / a;
        if temp2 > t_min && temp2 < t_max {
            rec.t = temp2;
            rec.p = ray.point_at_parameter(rec.t);
            rec.normal = (&rec.p - &self.centre(ray.time)) / self.radius;
            rec.material = self.material;
            return true;
        }

        false
    }
}

// #[derive(Copy, Clone)]
// pub struct Cylinder {
    // pub centre: f32
//     pub radius: f32,
//     pub phi_max: f32,
//     pub zMin: f32,
//     pub zMax: f32,
//     pub material: Material,
// }

impl Hitable for Cylinder {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc : Vec3 = ray.origin() - &self.centre;

        // let a : f32 = dot(&ray.direction(), &ray.direction());
        let a : f32 = &ray.b.x() * &ray.b.x() + &ray.b.y() + &ray.b.y();
        // let b : f32 = dot(&oc, &ray.direction());
        let b : f32 = 2.0 * &oc.x() * &ray.b.x() + &oc.y() + &ray.b.y();
        let c : f32 = &oc.x() * &oc.x() + &oc.y() + &oc.y() - self.radius * self.radius;

        let discriminant : f32 = b * b - a * c;

        if discriminant <= 0.0 {
            return false;
        }

        let temp : f32 = (-b - (b * b - a * c).sqrt()) / a;

        let t0 : f32 = if temp > t_min && temp < t_max {
            temp
        } else {
            (-b + (b * b - a * c).sqrt()) / a
        };

        if t0 > t_min && t0 < t_max {
            let mut hit = ray.point_at_parameter(t0);

            let hit_rad = (hit.x() * hit.x() + hit.y() * hit.y()).sqrt();
            hit.e[0] *= self.radius / hit_rad;
            hit.e[1] *= self.radius / hit_rad;

            // let mut phi = std::atan2(hit.y(), hit.x()).;
            let mut phi = hit.y().atan2(hit.x());
            
            if (phi < 0.0) {
                phi += 2.0 * f32::consts::PI;
            }

            if (hit.z() < self.zMin || hit.z() > self.zMax || phi > self.phi_max) {
                return false;
                // if (tShapeHit == t0) {
                //     return false;
                // }
                // tShapeHit = t0;
                
                // if (t0.UpperBound() > ray.tMax) {
                //     return false;
                // }
                // // <<Compute cylinder hit point and >> 
                // if (hit.z() < self.zMin || hit.z() > self.zMax || phi > self.phi_max)
                    // return false;
           }

            let u = phi / self.phi_max;
            let v = (hit.z() - self.zMin) / (self.zMax - self.zMin);

            // Vector3f dpdu(-self.phi_max * hit.y(), self.phi_max * hit.x(), 0);
            // Vector3f dpdv(0, 0, zMax - zMin);

            let dpdu = Vec3 { e: [-self.phi_max * hit.y(), self.phi_max * hit.x(), 0.0]};
            let dpdv = Vec3 { e: [0.0, 0.0, self.zMax - self.zMin]};

            let d2Pduu = -self.phi_max * self.phi_max * Vec3 { e: [hit.x(), hit.y(), 0.0]};
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

            rec.t = t0;
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

#[derive(Clone)]
pub struct HitList {
    pub spheres : Vec<Sphere>,
    pub moving_spheres : Vec<MovingSphere>,
    pub cylinders : Vec<Cylinder>,
}

impl Hitable for HitList {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit_rec : HitRecord = HitRecord {
            t: t_max,
            p: Vec3 { e: [0.0, 0.0, 0.0]},
            normal: Vec3 { e: [0.0, 0.0, 0.0]},
            material: Material::make_dummy_material(),
        };
        let mut hit_anything : bool = false;
        let mut closest_so_far : f32 = t_max;

        for hit_item in self.spheres.iter() {
            if hit_item.hit(ray, t_min, closest_so_far, &mut hit_rec) {
                hit_anything = true;
                closest_so_far = hit_rec.t;
                rec.t = hit_rec.t;
                rec.p = hit_rec.p.clone();
                rec.normal = hit_rec.normal.clone();
                rec.material = hit_rec.material.clone();
            }
        }

        for hit_item in self.moving_spheres.iter() {
            if hit_item.hit(ray, t_min, closest_so_far, &mut hit_rec) {
                hit_anything = true;
                closest_so_far = hit_rec.t;
                rec.t = hit_rec.t;
                rec.p = hit_rec.p.clone();
                rec.normal = hit_rec.normal.clone();
                rec.material = hit_rec.material.clone();
            }
        }

        for hit_item in self.cylinders.iter() {
            if hit_item.hit(ray, t_min, closest_so_far, &mut hit_rec) {
                hit_anything = true;
                closest_so_far = hit_rec.t;
                rec.t = hit_rec.t;
                rec.p = hit_rec.p.clone();
                rec.normal = hit_rec.normal.clone();
                rec.material = hit_rec.material.clone();
            }
        }

        hit_anything
    }
}
