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

#[derive(Copy, Clone)]
pub struct Disc {
    pub centre : Vec3,
    pub radius: f32,
    pub height: f32,
    pub innerRadius: f32,
    pub phiMax : f32,
    pub material: Material,
}

#[derive(Copy, Clone)]
pub struct BoxShape {
    pub corner_1 : Vec3,
    pub corner_2 : Vec3,

    pub side_1: RectXY,
    pub side_2: RectXY,
    pub side_3: RectXZ,
    pub side_4: RectXZ,
    pub side_5: RectYZ,
    pub side_6: RectYZ,
}

impl BoxShape {
    fn create(
        centre : Vec3,
        x_length : f32,
        y_length : f32,
        z_length : f32,
        material: Material,
    ) -> BoxShape {
        let x0 = centre.x() - x_length / 2.0;
        let x1 = centre.x() + x_length / 2.0;
        let y0 = centre.y() - y_length / 2.0;
        let y1 = centre.y() + y_length / 2.0;
        let z0 = centre.z() - z_length / 2.0;
        let z1 = centre.z() + z_length / 2.0;

        BoxShape {
            corner_1: Vec3 { e: [x0, y0, z0]},
            corner_2: Vec3 { e: [x1, y1, z1]},
            side_1: RectXY {
                x0: x0,
                x1: x1,
                y0: y0,
                y1: y1,
                z: z0,
                material: material,
            },
            side_2: RectXY {
                x0: x0,
                x1: x1,
                y0: y0,
                y1: y1,
                z: z1,
                material: material,
            },
            side_3: RectXZ {
                x0: x0,
                x1: x1,
                z0: z0,
                z1: z1,
                y: y0,
                material: material,
            },
            side_4: RectXZ {
                x0: x0,
                x1: x1,
                z0: z0,
                z1: z1,
                y: y1,
                material: material,
            },
            side_5: RectYZ {
                y0: y0,
                y1: y1,
                z0: z0,
                z1: z1,
                x: x0,
                material: material,
            },
            side_6: RectYZ {
                y0: y0,
                y1: y1,
                z0: z0,
                z1: z1,
                x: x1,
                material: material,
            },
        }
    }
}

#[derive(Copy, Clone)]
pub struct RectXY {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub z: f32,

    pub material: Material,
}

#[derive(Copy, Clone)]
pub struct RectXZ {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub y: f32,

    pub material: Material,
}

#[derive(Copy, Clone)]
pub struct RectYZ {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub x: f32,

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

fn solve_quadratic(a: f32, b: f32, c: f32) -> (bool, f32, f32) {
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

impl Hitable for Disc {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        if ray.b.z() == 0.0 {
           return false;
        }

        let shape_hit = (self.height - ray.a.z()) / ray.b.z();
        if shape_hit <= 0.0 || shape_hit >= t_max {
            return false;
        }

        let hit = ray.point_at_parameter(shape_hit);
        let dist2 = hit.x() * hit.x() + hit.y() * hit.y();

        if dist2 > self.radius * self.radius || dist2 < self.innerRadius * self.innerRadius {
            return false;
        }

        let mut phi = hit.y().atan2(hit.x());
        if phi < 0.0 {
            phi += 2.0 * f32::consts::PI;
        }

        if phi > self.phiMax {
            return false;
        }

        rec.t = shape_hit;
        rec.p = hit;
        // rec.normal = N;
        rec.normal = (&rec.p - &self.centre) / self.radius;
        rec.normal = ray.a - &rec.p;
        rec.material = self.material;
        return true;
    }
}

impl Hitable for BoxShape {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut updating_t_max = t_max;
        let mut hit = false;
        if self.side_1.hit(ray, t_min, updating_t_max, rec) {
            hit = true;
            updating_t_max = rec.t;
        }
        if self.side_2.hit(ray, t_min, updating_t_max, rec) {
            hit = true;
            updating_t_max = rec.t;
        }
        if self.side_3.hit(ray, t_min, updating_t_max, rec) {
            hit = true;
            updating_t_max = rec.t;
        }
        if self.side_4.hit(ray, t_min, updating_t_max, rec) {
            hit = true;
            updating_t_max = rec.t;
        }
        if self.side_5.hit(ray, t_min, updating_t_max, rec) {
            hit = true;
            updating_t_max = rec.t;
        }
        if self.side_6.hit(ray, t_min, updating_t_max, rec) {
            hit = true;
        }
        return hit;
    }
}

impl Hitable for RectXY {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        if ray.b.z() == 0.0 {
            return false;
        }

        let t_hit = (self.z - ray.a.z()) / ray.b.z();
        if t_hit < t_min || t_hit > t_max {
            return false;
        }

        let mut hit = ray.point_at_parameter(t_hit);

        if hit.x() < self.x0 ||
           hit.x() > self.x1 ||
           hit.y() < self.y0 ||
           hit.y() > self.y1 {
            return false;
        }

        rec.t = t_hit;
        rec.p = hit;
        let z_normal = if ray.b.z() < 0.0 { 1.0 } else { -1.0 };
        rec.normal = Vec3 { e: [0.0, 0.0, z_normal]};
        rec.material = self.material;

        true
    }
}

impl Hitable for RectXZ {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        if ray.b.y() == 0.0 {
            return false;
        }

        let t_hit = (self.y - ray.a.y()) / ray.b.y();
        if t_hit < t_min || t_hit > t_max {
            return false;
        }

        let mut hit = ray.point_at_parameter(t_hit);

        if hit.x() < self.x0 ||
           hit.x() > self.x1 ||
           hit.z() < self.z0 ||
           hit.z() > self.z1 {
            return false;
        }

        rec.t = t_hit;
        rec.p = hit;
        let y_normal = if ray.b.y() < 0.0 { 1.0 } else { -1.0 };
        rec.normal = Vec3 { e: [0.0, y_normal, 0.0]};
        rec.material = self.material;

        true
    }
}

impl Hitable for RectYZ {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        if ray.b.x() == 0.0 {
            return false;
        }

        let t_hit = (self.x - ray.a.x()) / ray.b.x();
        if t_hit < t_min || t_hit > t_max {
            return false;
        }

        let mut hit = ray.point_at_parameter(t_hit);

        if hit.y() < self.y0 ||
           hit.y() > self.y1 ||
           hit.z() < self.z0 ||
           hit.z() > self.z1 {
            return false;
        }

        rec.t = t_hit;
        rec.p = hit;
        let x_normal = if ray.b.x() < 0.0 { 1.0 } else { -1.0 };
        rec.normal = Vec3 { e: [x_normal, 0.0, 0.0]};
        rec.material = self.material;

        true
    }
}

#[derive(Clone)]
pub struct HitList {
    pub spheres : Vec<Sphere>,
    pub moving_spheres : Vec<MovingSphere>,
    pub cylinders : Vec<Cylinder>,
    pub discs : Vec<Disc>,
    pub boxes : Vec<BoxShape>,
    pub rect_xy : Vec<RectXY>,
    pub rect_xz : Vec<RectXZ>,
    pub rect_yz : Vec<RectYZ>,
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

        for hit_item in self.boxes.iter() {
            if hit_item.hit(ray, t_min, closest_so_far, &mut hit_rec) {
                hit_anything = true;
                closest_so_far = hit_rec.t;
                rec.t = hit_rec.t;
                rec.p = hit_rec.p.clone();
                rec.normal = hit_rec.normal.clone();
                rec.material = hit_rec.material.clone();
            }
        }

        for hit_item in self.rect_xy.iter() {
            if hit_item.hit(ray, t_min, closest_so_far, &mut hit_rec) {
                hit_anything = true;
                closest_so_far = hit_rec.t;
                rec.t = hit_rec.t;
                rec.p = hit_rec.p.clone();
                rec.normal = hit_rec.normal.clone();
                rec.material = hit_rec.material.clone();
            }
        }

        for hit_item in self.rect_xz.iter() {
            if hit_item.hit(ray, t_min, closest_so_far, &mut hit_rec) {
                hit_anything = true;
                closest_so_far = hit_rec.t;
                rec.t = hit_rec.t;
                rec.p = hit_rec.p.clone();
                rec.normal = hit_rec.normal.clone();
                rec.material = hit_rec.material.clone();
            }
        }

        for hit_item in self.rect_yz.iter() {
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
