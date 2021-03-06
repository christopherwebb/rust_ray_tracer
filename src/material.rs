use rand::Rng;

use serde::{Deserialize, Serialize};

use crate::core::{
    Point3f,
    Vector3f,
    Normal3f,
    Colour,
    dot_vn,
    reflect,
    refract,
};

use crate::ray::Ray;


#[derive(Clone)]
pub struct HitRecord {
    pub t : f32,
    pub p : Point3f,
    pub normal : Normal3f,
    pub material: Material,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
}

pub struct MaterialHit {
    pub hit : bool,
    pub atten: Colour,
    pub ray_out: Ray,
    // material : Material,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Material {
    mat_type : MaterialType,
    albedo : Colour,
    fuzz : f32,
    ref_idx : f32,
}

impl Material {
    pub fn scatter(self, ray_in: &Ray, hit: &HitRecord) -> MaterialHit {
        match self.mat_type {
            MaterialType::Lambertian => {
                let target = hit.p + hit.normal + Vector3f::rnd_in_unit_sphere();

                MaterialHit {
                    hit : true,
                    atten : self.albedo,
                    ray_out : Ray {
                        a: hit.p,
                        b: Vector3f::from(target - hit.p),
                        time: ray_in.time,
                    },
                }
            },
            MaterialType::Metal => {
                let reflected = reflect(
                    &ray_in.direction().unit_vector(),
                    &hit.normal
                );
                let scattered : Ray = Ray {
                    a: hit.p,
                    b: reflected + self.fuzz * Vector3f::rnd_in_unit_sphere(),
                    time: ray_in.time,
                };

                MaterialHit {
                    hit : dot_vn(&scattered.direction(), &hit.normal) > 0.0,
                    atten : self.albedo,
                    ray_out : scattered,
                }
            },
            MaterialType::Dielectric => {
                let reflected = reflect(&ray_in.direction(), &hit.normal);

                let atten = Colour { r: 1.0, g: 1.0, b: 1.0 };

                let dot_prod : f32 = dot_vn(&ray_in.direction(), &hit.normal);
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
                            time: ray_in.time,
                        },
                    }
                } else {
                    MaterialHit {
                        hit : true,
                        atten : atten,
                        ray_out : Ray {
                            a: hit.p,
                            b: reflected,
                            time: ray_in.time,
                        },
                    }
                }
            },
        }
    }

    pub fn schlick(cosine : f32, ref_idx : f32) -> f32 {
        let r0 : f32 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0_sqr = r0 * r0;

        r0_sqr + (1.0 - r0_sqr) * (1.0 - cosine).powf(5.0)
    }

    pub fn make_lambertian(albedo: Colour) -> Material {
        Material {
            mat_type: MaterialType::Lambertian,
            albedo: albedo,
            fuzz: 0.0,
            ref_idx: 0.0,
        }
    }
    pub fn make_metal(albedo: Colour, fuzz: f32) -> Material {
        Material {
            mat_type: MaterialType::Metal,
            albedo: albedo,
            fuzz: fuzz,
            ref_idx: 0.0,
        }
    }
    pub fn make_dielectric(ref_idx: f32) -> Material {
        Material {
            mat_type: MaterialType::Dielectric,
            albedo: Colour { r: 0.0, g: 0.0, b: 0.0 },
            fuzz: 0.0,
            ref_idx: ref_idx,
        }
    }
    pub fn make_dummy_material() -> Material {
        Material {
            mat_type: MaterialType::Metal,
            albedo: Colour { r: 0.0, g: 0.0, b: 0.0 },
            fuzz: 0.0,
            ref_idx: 0.0,
        }
    }
}
