use crate::material::{
    Material,
    HitRecord
};

use crate::ray::Ray;

use crate::vector::{
    Vec3,
    dot,
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

#[derive(Clone)]
pub struct HitList {
    pub spheres : Vec<Sphere>,
    pub moving_spheres : Vec<MovingSphere>,
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

        hit_anything
    }
}
