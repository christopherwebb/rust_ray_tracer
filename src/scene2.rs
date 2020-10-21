use crate::ray::Ray;
use crate::shapes::base::{Interaction, ShapeTrait};
use crate::primative::Primative;
use crate::core::{Point3f, Normal3f, Colour};
use crate::camera::Camera;


const T_MAX: f32 = 1000000.0;

pub struct Scene {
    pub primatives: Vec<Primative>,
    pub camera: Camera,
}

struct PrimativeInteraction {
    primative: usize,
    interaction: Interaction,
}

impl Scene {
    fn find_interaction(&self, ray: &Ray) -> Option<PrimativeInteraction> {
        let mut t_max = T_MAX;

        // let mut nearest_hit = 
        // let mut closest: Option<Primative>;
        // let mut ray_hit: Option<Ray>;

        // let mut closest: Option<PrimativeInteraction> = None;
        // for (index, primative) in self.primatives.iter().enumerate() {
        //     let interaction_option = primative.collide(ray, 0.0, t_max);

        //     match interaction_option {
        //         Some(interaction) => {
        //             closest = Some(PrimativeInteraction{
        //                 primative: index,
        //                 interaction: *interaction,
        //             });
        //             t_max = interaction.t;
        //         },
        //         None => (),
        //     }
        // }

        let ray = ray.clone();

        return (&self.primatives).iter().enumerate().fold(None, |previous, (index, primative)| {
            // let index = enum_prim_tuple[0];
            // let primative = enum_prim_tuple[1];
            let prima_clone = primative.clone();

            let interaction_option = prima_clone.collide(ray, 0.0, t_max);
            match interaction_option {
                Some(interaction) => {
                    t_max = interaction.t;
                    Some(PrimativeInteraction{
                        primative: index,
                        interaction,
                    })
                },
                None => previous,
            }
        });
    }
}

pub fn calculate_colour(scene: &Scene, ray: &Ray, depth: i32) -> Colour {
    // let unit_dir = ray.direction().unit_vector();
    // let t : f32 = 0.5 * (unit_dir.y + 1.0);
    // (1.0 - t) * Colour { r: 1.0, g: 1.0, b: 1.0 } + t * Colour { r: 0.5, g: 0.7, b: 1.0 }
    
    let interaction_option = scene.find_interaction(ray);

    match interaction_option {
        Some(interaction) => {
            // let scatter_result = interaction.primative.scatter(ray, interaction.interaction);
            Colour {r: 1.0, g: 1.0, b: 1.0}
        },
        None => {
            let unit_dir = ray.direction().unit_vector();
            let t : f32 = 0.5 * (unit_dir.y + 1.0);
            (1.0 - t) * Colour { r: 1.0, g: 1.0, b: 1.0 } + t * Colour { r: 0.5, g: 0.7, b: 1.0 }
        },
    }
}
