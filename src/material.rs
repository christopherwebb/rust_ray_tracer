use rand::Rng;

use crate::vector::{
    Vec3,
    dot,
    unit_vector,
    rnd_in_unit_sphere,
    reflect,
    refract,
};

use crate::ray::Ray;

#[derive(Clone)]
pub struct HitRecord {
    pub t : f32,
    pub p : Vec3,
    pub normal : Vec3,
    pub material: Material,
}

#[derive(Clone, Copy)]
enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
}

pub struct MaterialHit {
    pub hit : bool,
    pub atten: Vec3,
    pub ray_out: Ray,
    // material : Material,
}

#[derive(Clone, Copy)]
pub struct Material {
    mat_type : MaterialType,
    albedo : Vec3,
    fuzz : f32,
    ref_idx : f32,
}

impl Material {
    pub fn scatter(self, ray_in: &Ray, hit: &HitRecord) -> MaterialHit {
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
                let reflected : Vec3 = reflect(&ray_in.direction(), &hit.normal);

                let atten : Vec3 = Vec3 {e: [1.0, 1.0, 1.0]};

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

    pub fn schlick(cosine : f32, ref_idx : f32) -> f32 {
        let r0 : f32 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0_sqr = r0 * r0;

        r0_sqr + (1.0 - r0_sqr) * (1.0 - cosine).powf(5.0)
    }

    pub fn make_lambertian(albedo: Vec3) -> Material {
        Material {
            mat_type: MaterialType::Lambertian,
            albedo: albedo,
            fuzz: 0.0,
            ref_idx: 0.0,
        }
    }
    pub fn make_metal(albedo: Vec3, fuzz: f32) -> Material {
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
            albedo: Vec3 {e: [0.0, 0.0, 0.0]},
            fuzz: 0.0,
            ref_idx: ref_idx,
        }
    }
    pub fn make_dummy_material() -> Material {
        Material {
            mat_type: MaterialType::Metal,
            albedo: Vec3 {e: [0.0, 0.0, 0.0]},
            fuzz: 0.0,
            ref_idx: 0.0,
        }
    }
}