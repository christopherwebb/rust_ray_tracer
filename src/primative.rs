// use std::rc::Rc;
use std::sync::Arc;

use crate::ray::Ray;
use crate::shapes::base::{Interaction, ShapeTrait};
use crate::core::Transform;

#[derive(Clone)]
pub struct Primative {
    pub shape: Arc<dyn ShapeTrait + Send + Sync>,
    // pub material: Arc<dyn MaterialTrait>,
    pub transform: Arc<dyn TransformTrait + Send + Sync>,
}

impl Primative {
    pub fn collide(self, ray: Ray, t_min: f32, t_max: f32) -> Option<Interaction> {
        let transform = self.transform.generate_transform(ray.time);
        let interaction_result = self.shape.collide(
            ray,
            transform.m,
            transform.m_inv,
            t_min,
            t_max,
        );

        interaction_result
    }

    // pub fn scatter(self, ray: Ray, interaction: Interaction) -> ScatterResult {
    //     self.material.scatter(ray, interaction)
    // }
}

pub trait TransformTrait {
    fn generate_transform(&self, time_t: f32) -> Transform;
}



#[cfg(test)]
mod sphere_tests {
    use std::sync::Arc;
    use crate::primative::{Primative, TransformTrait};
    use crate::shapes::sphere2::Sphere;
    use crate::core::{
        Vector3f,
        Point3f,
        Normal3f,
        Transform,
        gen_translate,
        gen_scale,
        // gen_rotate_x,
        // gen_rotate_y,
        // gen_rotate_z,
        gen_rotate,
    };
    use crate::ray::Ray;

    struct SRTTransform {
        scale: Transform,
        rotate: Transform,
        translate: Transform,
    }

    impl SRTTransform {
        fn init(
            delta: Vector3f,
            scale_x: f32,
            scale_y: f32,
            scale_z: f32,
            theta: f32,
            axis: Vector3f,
        ) -> SRTTransform {
            SRTTransform {
                scale: gen_scale(scale_x, scale_y, scale_z),
                rotate: gen_rotate(theta, axis),
                translate: gen_translate(delta),
            }
        }
    }

    impl TransformTrait for SRTTransform {
        fn generate_transform(&self, time_t: f32) -> Transform {
            self.translate * self.rotate * self.scale
        }
    }

    #[test]
    fn test_instantiate() {
        let primative = Primative {
            shape: Arc::new(Sphere {radius: 1.0}),
            transform: Arc::new(SRTTransform::init(
                Vector3f {x: 0.0, y: 0.0, z: 0.0},
                1.0,
                1.0,
                1.0,
                0.0,
                Vector3f {x: 0.0, y: 1.0, z: 0.0},
            )),
        };

        let ray = Ray {
            a: Point3f {x: 2.0, y: 0.0, z: 0.0},
            b: Vector3f {x: -2.0, y: 0.0, z: 0.0},
            time: 0.0,
        };
        let interaction = primative.collide(ray, 0.0, 10.0).unwrap();

        assert_eq!(interaction.t, 0.5);
        assert_eq!(interaction.p, Point3f {x: 1.0, y: 0.0, z: 0.0});
        assert_eq!(interaction.normal, Normal3f {x: 1.0, y: 0.0, z: 0.0});
    }
}
