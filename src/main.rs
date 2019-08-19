// use std::io::{self, Write};
// use std::thread;
use std::f32;
use rand::thread_rng;
use rand::Rng;
use rand::rngs::ThreadRng;
use num_cpus;

mod vector;
use crate::vector::{
    Vec3,
    dot,
    cross,
    unit_vector,
    rnd_in_unit_sphere,
    rnd_in_unit_disc,
    reflect,
    refract,
};

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
    material: Material,
}

// trait Material {
//     fn scatter(ray_in: &Ray, hit: &HitRecordy) -> MaterialHit;
// }

struct MaterialHit {
    hit : bool,
    atten: Vec3,
    ray_out: Ray,
    // material : Material,
}

#[derive(Clone, Copy)]
enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
}

#[derive(Clone, Copy)]
struct Material {
    mat_type : MaterialType,
    albedo : Vec3,
    fuzz : f32,
    ref_idx : f32,
}

impl Material {
    fn scatter(self, ray_in: &Ray, hit: &HitRecord) -> MaterialHit {
        match self.mat_type {
            MaterialType::Lambertian => {
                let target : Vec3 = hit.p + hit.normal + rnd_in_unit_sphere();

                MaterialHit {
                    hit : true,
                    atten : self.albedo,
                    ray_out : Ray {
                        a: hit.p,
                        b: target - hit.p,
                    },
                }
            },
            MaterialType::Metal => {
                let reflected : Vec3 = reflect(&unit_vector(&ray_in.direction()), &hit.normal);
                let scattered : Ray = Ray {
                    a: hit.p,
                    b: &reflected + &(self.fuzz * rnd_in_unit_sphere()),
                };

                MaterialHit {
                    hit : dot(&scattered.direction(), &hit.normal) > 0.0,
                    atten : self.albedo,
                    ray_out : scattered,
                }
            },
            MaterialType::Dielectric => {
                // let outward_normal  : Vec3;
                let reflected : Vec3 = reflect(&ray_in.direction(), &hit.normal);

                // let ni_over_nt : f32;

                let atten : Vec3 = Vec3 {e: [1.0, 1.0, 1.0]};

                // let refracted : Vec3;

                // if dot(&ray_in.direction(), &hit.normal) > 0.0 {
                //     outward_normal = -hit.normal;
                //     ni_over_nt = self.ref_idx;
                // } else {
                //     outward_normal = hit.normal;
                //     ni_over_nt = 1.0 / self.ref_idx;
                // }

                let dot_prod : f32 = dot(&ray_in.direction(), &hit.normal);
                let (outward_normal, ni_over_nt, cosine) =
                if dot_prod > 0.0 {
                    (
                        -hit.normal,
                        self.ref_idx,
                        self.ref_idx * dot_prod / &ray_in.direction().length()
                    )
                } else {
                    (
                        hit.normal,
                        1.0 / self.ref_idx,
                        -dot_prod / &ray_in.direction().length()
                    )
                };

                let (refracting, refracted) = refract(&ray_in.direction(), &outward_normal, ni_over_nt);
                if refracting {
                    let reflect_prob = Material::schlick(cosine, self.ref_idx);
                    let random : f32= rand::thread_rng().gen();

                    let ray = if random < reflect_prob {
                        reflected
                    } else {
                        refracted.unwrap()
                    };

                    MaterialHit {
                        hit : true,
                        atten : atten,
                        ray_out : Ray {
                            a: hit.p,
                            b: ray,
                        },
                    }
                } else {
                    MaterialHit {
                        hit : true,
                        atten : atten,
                        ray_out : Ray {
                            a: hit.p,
                            b: reflected,
                        },
                    }
                }
            },
        }
    }

    fn schlick(cosine : f32, ref_idx : f32) -> f32 {
        let r0 : f32 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0_sqr = r0 * r0;

        r0_sqr + (1.0 - r0_sqr) * (1.0 - cosine).powf(5.0)
    }

    fn make_lambertian(albedo: Vec3) -> Material {
        Material {
            mat_type: MaterialType::Lambertian,
            albedo: albedo,
            fuzz: 0.0,
            ref_idx: 0.0,
        }
    }
    fn make_metal(albedo: Vec3, fuzz: f32) -> Material {
        Material {
            mat_type: MaterialType::Metal,
            albedo: albedo,
            fuzz: fuzz,
            ref_idx: 0.0,
        }
    }
    fn make_dielectric(ref_idx: f32) -> Material {
        Material {
            mat_type: MaterialType::Dielectric,
            albedo: Vec3 {e: [0.0, 0.0, 0.0]},
            fuzz: 0.0,
            ref_idx: ref_idx,
        }
    }
    fn make_dummy_material() -> Material {
        Material {
            mat_type: MaterialType::Metal,
            albedo: Vec3 {e: [0.0, 0.0, 0.0]},
            fuzz: 0.0,
            ref_idx: 0.0,
        }
    }
}

// struct Lambertian {
//     albedo : vec3,
// // }

// // impl Material for Lambertian {
//     fn scatter(self, ray_in: &Ray, hit: &HitRecord) -> MaterialHit {

//         let target : Vec3 = hit.p + hit.normal + rnd_in_unit_sphere();
//         // let new_ray : Ray = Ray {
//         //             a: hit_rec.p,
//         //             b: target - &hit_rec.p,
//         //         };



//         MaterialHit {
//             hit : true,
//             atten : self.albedo,
//             ray_out : Ray {
//                 a: hit.p,
//                 b: target - hit.p,
//             }
//         }
//     }
// }

trait Hitable {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

#[derive(Copy, Clone)]
struct Sphere {
    pub centre : Vec3,
    pub radius: f32,
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

#[derive(Clone)]
struct HitList {
    list : Vec<Sphere>,
}

impl HitList {
    fn three_spheres_on_world() -> HitList {
        HitList {
            list: vec![
                Sphere {
                    centre: Vec3 { e: [0.0, 0.0, -1.0]},
                    radius: 0.5,
                    material: Material::make_lambertian(
                        Vec3 { e: [0.1, 0.2, 0.5]},
                    )
                },
                Sphere {
                    centre: Vec3 { e: [0.0, -100.5, -1.0]},
                    radius: 100.0,
                    material: Material::make_lambertian(
                        Vec3 { e: [0.8, 0.8, 0.0]},
                    )
                },
                Sphere {
                    centre: Vec3 { e: [1.0, 0.0, -1.0]},
                    radius: 0.5,
                    material: Material::make_metal(
                        Vec3 { e: [0.8, 0.6, 0.2]},
                        1.0,
                    )
                },
                Sphere {
                    centre: Vec3 { e: [-1.0, 0.0, -1.0]},
                    radius: 0.5,
                    material: Material::make_dielectric(1.5)
                },
                Sphere {
                    centre: Vec3 { e: [-1.0, 0.0, -1.0]},
                    radius: -0.45,
                    material: Material::make_dielectric(1.5)
                },
            ]
        }
    }
    fn blue_red_spheres() -> HitList {
        let sphere_radius: f32 = (f32::consts::PI / 4.0).cos();
        HitList {
            list: vec![
                Sphere {
                    centre: Vec3 { e: [-sphere_radius, 0.0, -1.0]},
                    radius: sphere_radius,
                    material: Material::make_lambertian(
                        Vec3 { e: [0.0, 0.0, 1.0]},
                    )
                },
                Sphere {
                    centre: Vec3 { e: [sphere_radius, 0.0, -1.0]},
                    radius: sphere_radius,
                    material: Material::make_lambertian(
                        Vec3 { e: [1.0, 0.0, 0.0]},
                    )
                },
            ]
        }
    }
    fn random_world(rng: &mut ThreadRng) -> HitList {
        let small_radius = 0.2;
        let large_radius = 1.0;
        let mut sphere_list = vec![];

        sphere_list.push(Sphere {
            centre: Vec3 { e: [0.0, -1000.0, 0.0]},
            radius: 1000.0,
            material: Material::make_lambertian(
                Vec3 { e: [0.5, 0.5, 0.5]},
            )
        });

        sphere_list.push(Sphere {
            centre: Vec3 { e: [0.0, 1.0, 0.0]},
            radius: large_radius,
            material: Material::make_dielectric(1.5)
        });
        sphere_list.push(Sphere {
            centre: Vec3 { e: [-4.0, 1.0, 0.0]},
            radius: large_radius,
            material: Material::make_lambertian(
                Vec3 { e: [0.4, 0.2, 0.1]},
            )
        });
        sphere_list.push(Sphere {
            centre: Vec3 { e: [4.0, 1.0, 0.0]},
            radius: large_radius,
            material: Material::make_metal(
                Vec3 { e: [0.7, 0.6, 0.5]},
                0.0,
            )
        });

        let distance_filter = Vec3 { e : [4.0, 0.2, 0.0]};

        for a in -11..11 {
            for b in -11..11 {
                let chosen_mat = rng.gen::<f64>();
                let centre = Vec3 { e: [
                    a as f32 + 0.9 * rng.gen::<f64>() as f32,
                    small_radius,
                    b as f32 + 0.9 * rng.gen::<f64>() as f32,
                ]};

                if (centre - distance_filter).length() > 0.9 {
                    sphere_list.push(Sphere {
                        centre: centre,
                        radius: 0.2,
                        material: match chosen_mat {
                            x if x < 0.8 => Material::make_lambertian(Vec3 { e: [
                                rng.gen::<f64>() as f32 * rng.gen::<f64>() as f32,
                                rng.gen::<f64>() as f32 * rng.gen::<f64>() as f32,
                                rng.gen::<f64>() as f32 * rng.gen::<f64>() as f32,
                            ]}),
                            x if x < 0.95 => Material::make_metal(Vec3 { e: [
                                0.5 * (1.0 + rng.gen::<f64>() as f32),
                                0.5 * (1.0 + rng.gen::<f64>() as f32),
                                0.5 * (1.0 + rng.gen::<f64>() as f32),
                            ]}, 0.5 * rng.gen::<f64>() as f32),
                            _ => Material::make_dielectric(1.5),
                        },
                    });
                }
            }
        }

        HitList {
            list: sphere_list
        }
    }
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

        for hit_item in self.list.iter() {
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

#[derive(Clone, Copy)]
struct Camera {
    origin : Vec3,
    lower_left_corner : Vec3,
    horizontal : Vec3,
    vertical : Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    fn create(
        look_from : Vec3,
        look_at : Vec3,
        up : Vec3,
        fvov : f32,
        aspect : f32,
        aperature: f32,
        focus_dist: f32,
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
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
        }
    }
    fn get_ray(self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * rnd_in_unit_disc();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            a: self.origin.clone() + offset,
            b: &self.lower_left_corner + &(s * &self.horizontal) + t * &self.vertical - &self.origin - offset
        }
    }
}

fn colour(ray : &Ray, world: &Hitable, depth : i32) -> Vec3 {
    let mut hit_rec : HitRecord = HitRecord {
            t: 10000.0,
            p: Vec3 { e: [0.0, 0.0, 0.0]},
            normal: Vec3 { e: [0.0, 0.0, 0.0]},
            material: Material::make_dummy_material(),
        };

    if world.hit(ray, 0.001, 10000.0, &mut hit_rec) {
        if depth >= 50 {
            return Vec3 { e: [0.0, 0.0, 0.0]};
        }

        let scatter_result : MaterialHit = hit_rec.material.scatter(&ray, &hit_rec);
        if !scatter_result.hit {
            return Vec3 { e: [0.0, 0.0, 0.0]};
        }

        return scatter_result.atten * colour(&scatter_result.ray_out, world, depth + 1);
    }

    let dir : Vec3 = ray.direction();
    let unit_dir : Vec3 = unit_vector(&dir);
    let t : f32 = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vec3 { e: [1.0, 1.0, 1.0]} + t * Vec3 { e: [0.5, 0.7, 1.0]}
}

fn main() {
    let n_x = 800;
    let n_y = 400;

    let aa_samples : u16 = 100;

    let mut rng = thread_rng();

    // let cam = Camera::create(
    //     Vec3 { e: [-2.0, 2.0,  1.0]},
    //     Vec3 { e: [ 0.0, 0.0, -1.0]},
    //     Vec3 { e: [ 0.0, 1.0,  0.0]},
    //     45.0,
    //     (n_x as f32) / (n_y as f32)
    // );

    let look_from = Vec3 { e: [ 13.0, 2.0, 3.0]};
    let look_at = Vec3 { e: [ 0.0, 0.0, 0.0]};
    let cam = Camera::create(
        look_from,
        look_at,
        Vec3 { e: [ 0.0, 1.0,  0.0]},
        20.0,
        (n_x as f32) / (n_y as f32),
        0.1,
        10.0,
    );

    // let cam = Camera::create(
    //     Vec3 { e: [ 0.0, 0.0,  0.0]},
    //     Vec3 { e: [ 0.0, 0.0, -1.0]},
    //     Vec3 { e: [ 0.0, 1.0,  0.0]},
    //     90.0,
    //     (n_x as f32) / (n_y as f32)
    // );

    let world : HitList = HitList::random_world(&mut rng);

    let aa_division : f32 = f32::from(aa_samples);

    println!("P3\n{} {}\n255", n_x, n_y);
    for y_coord in (0..n_y).rev() {
        for x_coord in 0..n_x {
            let mut col_sum = Vec3 { e: [0.0, 0.0, 0.0]};
            for aa_iter in 0..aa_samples {
                let rand_x : f32 = rng.gen::<f64>() as f32;
                let rand_y : f32 = rng.gen::<f64>() as f32;

                let u: f32 = (rand_x + x_coord as f32) / n_x as f32;
                let v: f32 = (rand_y + y_coord as f32) / n_y as f32;

                let ray = &cam.get_ray(u, v);

                col_sum += colour(&ray, &world, 0);
            }
            let col : Vec3 = col_sum / aa_division;

            let ir = (255.99 * col.r()) as u64;
            let ig = (255.99 * col.g()) as u64;
            let ib = (255.99 * col.b()) as u64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
