use serde::{Deserialize, Serialize};

use crate::core::{Point3f, Normal3f};
use crate::camera::Camera;
use crate::shapes::base::Hitable;
use crate::shapes::cylinder::Cylinder;
use crate::shapes::sphere::{Sphere, MovingSphere};

use crate::ray::Ray;

use crate::material::{
    Material,
    HitRecord
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Scene {
	pub hitlist : HitList,
	pub camera : Camera,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HitList {
    pub spheres : Vec<Sphere>,
    pub moving_spheres : Vec<MovingSphere>,
    pub cylinders : Vec<Cylinder>,
}

impl Hitable for HitList {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit_rec : HitRecord = HitRecord {
            t: t_max,
            p: Point3f { x: 0.0, y: 0.0, z: 0.0 },
            normal: Normal3f { x: 0.0, y: 0.0, z: 0.0 },
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
