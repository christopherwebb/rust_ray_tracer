use rand::Rng;

use serde::{Deserialize, Serialize};

use std::sync::Arc;

use crate::core::{
    Point3f,
    Vector3f,
    Normal3f,
    Colour,
    dot_vn,
    reflect,
    refract,
};

use crate::textures::base::Texture;
use crate::textures::solid_colour::SolidColour;

use crate::ray::Ray;

use crate::shapes::base::{Interaction};


struct HitRecord {}
pub struct ScatterResult {
    pub hit : bool,
    pub atten: Colour,
    pub ray_out: Ray,
}

pub trait MaterialTrait {
    // fn generate_response(&self) -> MaterialResult;
    // fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> ScatterResult;
    fn scatter(&self, ray_in: &Ray, interaction: &Interaction) -> ScatterResult;
}

// pub struct Lambertian<'a> {
pub struct Lambertian {
    // pub albedo: Colour,
    // pub albedo: &'a Texture,
    pub albedo: Arc<dyn Texture + Send + Sync>,
}

// impl MaterialTrait for Lambertian<'_> {
impl MaterialTrait for Lambertian {
    // fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> ScatterResult {
    fn scatter(&self, ray_in: &Ray, interaction: &Interaction) -> ScatterResult {
        let target = interaction.p + interaction.normal + Vector3f::rnd_in_unit_sphere();

        ScatterResult {
            hit : true,
            atten : self.albedo.value(interaction.u, interaction.v, interaction.p),
            ray_out : Ray {
                a: interaction.p,
                b: Vector3f::from(target - interaction.p),
                time: ray_in.time,
            },
        }
    }
}

impl Lambertian {
    // pub fn colour<'a>(colour: Colour) -> Lambertian<'a> {
    pub fn colour(colour: Colour) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColour {
                colour: colour
            })
        }
    }
}

pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f32,
}

impl MaterialTrait for Metal {
    // fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> ScatterResult {
    fn scatter(&self, ray_in: &Ray, interaction: &Interaction) -> ScatterResult {
        let reflected = reflect(
            &ray_in.direction().unit_vector(),
            &interaction.normal
        );
        let scattered : Ray = Ray {
            a: interaction.p,
            b: reflected + self.fuzz * Vector3f::rnd_in_unit_sphere(),
            time: ray_in.time,
        };

        ScatterResult {
            hit : dot_vn(&scattered.direction(), &interaction.normal) > 0.0,
            atten : self.albedo,
            ray_out : scattered,
        }
    }
}

pub struct Dielectric {
    pub ref_idx: f32,
}

impl MaterialTrait for Dielectric {
    // fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> ScatterResult {
    fn scatter(&self, ray_in: &Ray, interaction: &Interaction) -> ScatterResult {
        let reflected = reflect(&ray_in.direction(), &interaction.normal);

        let atten = Colour { r: 1.0, g: 1.0, b: 1.0 };

        let dot_prod : f32 = dot_vn(&ray_in.direction(), &interaction.normal);
        let (outward_normal, ni_over_nt, cosine) =
        if dot_prod > 0.0 {
            (
                -interaction.normal,
                self.ref_idx,
                self.ref_idx * dot_prod / &ray_in.direction().length()
            )
        } else {
            (
                interaction.normal,
                1.0 / self.ref_idx,
                -dot_prod / &ray_in.direction().length()
            )
        };

        let (refracting, refracted) = refract(&ray_in.direction(), &outward_normal, ni_over_nt);
        if refracting {
            let reflect_prob = schlick(cosine, self.ref_idx);
            let random: f32 = rand::thread_rng().gen();

            let ray = if random < reflect_prob {
                reflected
            } else {
                refracted.unwrap()
            };

            ScatterResult {
                hit : true,
                atten : atten,
                ray_out : Ray {
                    a: interaction.p,
                    b: ray,
                    time: ray_in.time,
                },
            }
        } else {
            ScatterResult {
                hit : true,
                atten : atten,
                ray_out : Ray {
                    a: interaction.p,
                    b: reflected,
                    time: ray_in.time,
                },
            }
        }
    }
}

// pub struct NormalMaterial {}

// impl MaterialTrait for NormalMaterial {
//     fn scatter(&self, ray_in: &Ray, interaction: &Interaction) -> ScatterResult {
//         ScatterResult {
//             hit: false,
//             atten: 0.5 * (Colour {r: interaction.normal.x, g: interaction.normal.y, b: interaction.normal.z} + Colour {r: 1.0, g: 1.0, b: 1.0}),
//             time: ray_in.time,
//         }
//     }

//     fn emm
// }

pub fn schlick(cosine : f32, ref_idx : f32) -> f32 {
    let r0: f32 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0_sqr = r0 * r0;

    r0_sqr + (1.0 - r0_sqr) * (1.0 - cosine).powf(5.0)
}
