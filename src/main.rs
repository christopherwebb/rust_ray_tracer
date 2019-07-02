use std::io::{self, Write};

mod vector;
use crate::vector::{Vec3, dot, cross, unit_vector};

struct Ray {
    a : Vec3,
    b : Vec3,
}

impl Ray {
    fn origin(&self) -> Vec3 { self.a.clone() }
    fn direction(&self) -> Vec3 { self.b.clone() }
    fn point_at_parameter(&self, point : f32) -> Vec3 { &self.a + &(point * &self.b) }
}

#[derive(Clone)]
struct HitRecord {
    t : f32,
    p : Vec3,
    normal : Vec3,
}

struct Sphere {
    centre : Vec3,
    radius: f32
}

trait Hitable {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
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
            return true;
        }

        let temp2 : f32 = (-b + (b * b - a * c).sqrt()) / a;
        if temp2 > t_min && temp2 < t_max {
            rec.t = temp2;
            rec.p = ray.point_at_parameter(rec.t);
            rec.normal = (&rec.p - &self.centre) / self.radius;
            return true;
        }

        false
    }
}

struct HitList {
    list : Vec<Sphere>,
}

impl Hitable for HitList {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit_rec : HitRecord = HitRecord {
            t: t_max,
            p: Vec3 { e: [0.0, 0.0, 0.0]},
            normal: Vec3 { e: [0.0, 0.0, 0.0]},
        };
        // let mut hit_rec_ref : &mut HitRecord = &mut hit_rec;
        let mut hit_anything : bool = false;
        let mut closest_so_far : f32 = t_max;

        for hit_item in self.list.iter() {
            // if hit_item.hit(ray, t_min, closest_so_far, hit_rec_ref) {
            if hit_item.hit(ray, t_min, closest_so_far, &mut hit_rec) {
                hit_anything = true;
                closest_so_far = hit_rec.t;
                // rec = &mut hit_rec.clone();
                rec.t = hit_rec.t;
                rec.p = hit_rec.p.clone();
                rec.normal = hit_rec.normal.clone();
            }
        }

        hit_anything
    }
}

fn hit_sphere(centre: &Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc : Vec3 = ray.origin() - centre;

    let a : f32 = dot(&ray.direction(), &ray.direction());
    let b : f32 = 2.0 * dot(&oc, &ray.direction());
    let c : f32 = dot(&oc, &oc) - radius * radius;

    let discriminant : f32 = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    }
    return (-b - discriminant.sqrt()) / (2.0 * a);
}

fn colour(ray : &Ray, world: &Hitable) -> Vec3 {
    let mut hit_rec : HitRecord = HitRecord {
            t: 10000.0,
            p: Vec3 { e: [0.0, 0.0, 0.0]},
            normal: Vec3 { e: [0.0, 0.0, 0.0]},
        };

    if world.hit(ray, 0.0, 10000.0, &mut hit_rec) {
        // let normal : Vec3 = unit_vector(ray.point_at_parameter(sphere_vec))
        //     + Vec3{ e: [1.0, 1.0, 1.0]};
        // return 0.5 * Vec3 { e: [
        //     normal.x() + 1.0,
        //     normal.y() + 1.0,
        //     normal.z() + 1.0,
        // ]};
        return 0.5 * (hit_rec.normal + Vec3 { e: [1.0, 1.0, 1.0]});
    }

    // let sphere_vec : f32 = hit_sphere(&Vec3 { e: [0.0, 0.0, -1.0]}, 0.5, r);
    // if sphere_vec > 0.0 {
    //     let normal : Vec3 = unit_vector(r.point_at_parameter(sphere_vec)) - Vec3{ e: [0.0, 0.0, -1.0]};
    //     return 0.5 * Vec3 { e: [
    //         normal.x() + 1.0,
    //         normal.y() + 1.0,
    //         normal.z() + 1.0,
    //     ]};
    // }

    let dir : Vec3 = ray.direction();
    let unit_dir : Vec3 = unit_vector(dir);
    let t : f32 = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vec3 { e: [1.0, 1.0, 1.0]} + t * Vec3 { e: [0.5, 0.7, 1.0]}
}

fn main() {
    let nX = 800;
    let nY = 400;

    let lower_left_corner : Vec3 = Vec3 { e: [-2.0, -1.0, -1.0]};
    let horizontal : Vec3 = Vec3 { e: [4.0, 0.0, 0.0]};
    let vertical : Vec3 = Vec3 { e: [0.0, 2.0, 0.0]};
    let origin : Vec3 = Vec3 { e: [0.0, 0.0, 0.0]};

    let world : HitList = HitList {
        list: vec![
            Sphere {
                centre: Vec3 { e: [0.0, 0.0, -1.0]}, 
                radius: 0.5,
            },
            Sphere {
                centre: Vec3 { e: [0.0, -100.5, -1.0]}, 
                radius: 100.0,
            },
        ]
    };

    println!("P3\n{} {}\n255", nX, nY);
    for y_coord in (0..nY).rev() {
        for x_coord in 0..nX {
            let u: f32 = x_coord as f32 / nX as f32;
            let v: f32 = y_coord as f32 / nY as f32;

            let ray : Ray = Ray {
                a: origin.clone(),
                b: lower_left_corner.clone() + u * horizontal.clone() + v * vertical.clone()
            };

            let col: Vec3 = colour(&ray, &world);

            let ir = (255.99 * col.r()) as u64;
            let ig = (255.99 * col.g()) as u64;
            let ib = (255.99 * col.b()) as u64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
