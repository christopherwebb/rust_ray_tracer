// use std::rc::Rc;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::ray::Ray;
use crate::shapes::base::{Interaction, ShapeTrait};
use crate::core::{Point3f, Transform};
use crate::material2::{MaterialTrait, ScatterResult};


#[derive(Clone)]
pub struct Primative {
    pub shape: Arc<dyn ShapeTrait + Send + Sync>,
    pub material: Arc<dyn MaterialTrait + Send + Sync>,
    pub transform: Arc<dyn TransformTrait + Send + Sync>,
}

impl Primative {
    pub fn collide(self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Interaction> {
        let transform = self.transform.generate_transform(ray.time);

        let object_to_world = transform.m;
        let world_to_object = transform.m_inv;

        let interaction_result = self.shape.collide(
            &(world_to_object * ray),
            t_min,
            t_max,
        );

        match interaction_result {
            Some(interaction) => {
                Some(Interaction {
                    t: interaction.t,
                    p: object_to_world * interaction.p,
                    normal: (world_to_object.transpose() * interaction.normal).unit_vector(),
                })
            },
            None => None,
        }
    }

    pub fn bounding_box(
        &self,
        time_0: f32,
        time_1: f32,
    ) -> Option<AABB> {
        let transform = self.transform.generate_transform(time_0);
        let object_to_world = transform.m;

        match self.shape.bounding_box(time_0, time_1) {
            Some(bounding) => {
                let transformed_min = object_to_world * bounding.minimum;
                let transformed_max = object_to_world * bounding.maximum;

                Some(AABB {
                    minimum: Point3f {
                        x: transformed_min.x.min(transformed_max.x),
                        y: transformed_min.y.min(transformed_max.y),
                        z: transformed_min.z.min(transformed_max.z),
                    },
                    maximum: Point3f {
                        x: transformed_min.x.max(transformed_max.x),
                        y: transformed_min.y.max(transformed_max.y),
                        z: transformed_min.z.max(transformed_max.z),
                    },
                })
            },
            None => None
        }
    }

    pub fn scatter(self, ray: &Ray, interaction: &Interaction) -> ScatterResult {
        self.material.scatter(ray, interaction)
    }
}

pub trait TransformTrait {
    fn generate_transform(&self, time_t: f32) -> Transform;
}



#[cfg(test)]
mod sphere_tests {
    use std::sync::Arc;
    use crate::shapes::base::{Interaction, ShapeTrait};
    use crate::primative::{Primative, TransformTrait};
    use crate::shapes::sphere2::Sphere;
    use crate::core::{
        Vector3f,
        Point3f,
        Normal3f,
        Colour,
        Transform,
        gen_translate,
        gen_scale,
        // gen_rotate_x,
        // gen_rotate_y,
        // gen_rotate_z,
        gen_rotate,
    };
    use crate::material2::{MaterialTrait, ScatterResult};
    use crate::ray::Ray;

    use float_cmp::approx_eq;

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

    struct DummyMaterial {}
    impl MaterialTrait for DummyMaterial {
        fn scatter(&self, ray_in: &Ray, interaction: &Interaction) -> ScatterResult {
            ScatterResult {
                hit: false,
                atten: Colour {r: 0.0, g: 0.0, b: 0.0},
                ray_out: Ray {
                    a: Point3f {x: 2.0, y: 0.0, z: 0.0},
                    b: Vector3f {x: -2.0, y: 0.0, z: 0.0},
                    time: 0.0,
                },
            }
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
            material: Arc::new(DummyMaterial {}),
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

    #[test]
    fn test_unit_sphere_normals() {
        let primative_original = Primative {
            shape: Arc::new(Sphere {radius: 1.0}),
            transform: Arc::new(SRTTransform::init(
                Vector3f {x: 0.0, y: 0.0, z: 0.0},
                1.0,
                1.0,
                1.0,
                0.0,
                Vector3f {x: 0.0, y: 1.0, z: 0.0},
            )),
            material: Arc::new(DummyMaterial {}),
        };

        for y in -6..6 {
            for x in -6..6 {
                let primative = primative_original.clone();

                let x = (x as f32) / 5.0;
                let y = (y as f32) / 5.0;

                let ray = Ray {
                    a: Point3f  {x:   x, y:   y, z:  4.0},
                    b: Vector3f {x: 0.0, y: 0.0, z: -1.0},
                    time: 0.0,
                };
                let interaction_result = primative.collide(ray, 0.0, 10.0);

                if x * x + y * y > 1.0 {
                    // miss case
                    assert!(interaction_result.is_none(), "Expected failure from x:{}, y:{}", x, y);
                } else {
                    let interaction = interaction_result.expect(&format!("Expected success from x:{}, y:{}", x, y));

                    let z_squared = (1.0 - x * x - y * y).max(0.0);
                    let z_point = z_squared.sqrt();

                    let expected = Point3f {x: x, y: y, z: z_point};
                    assert!(
                        approx_eq!(Point3f, interaction.p, expected, ulps = 4),
                        "Point comparison failed - returned {:?} expected {:?} (z squared: {})",
                        interaction.p,
                        expected,
                        z_squared,
                    );

                    let expected = Normal3f {x: x, y: y, z: z_point};
                    assert!(
                        approx_eq!(Normal3f, interaction.normal, expected),
                        "Normal comparison failed - returned {:?} expected {:?}",
                        interaction.normal,
                        expected,
                    );
                }
            }
        }
    }

    #[test]
    fn test_regularly_scaled_sphere_normals() {
        let primative_original = Primative {
            shape: Arc::new(Sphere {radius: 1.0}),
            transform: Arc::new(SRTTransform::init(
                Vector3f {x: 0.0, y: 0.0, z: 0.0},
                2.0,
                2.0,
                2.0,
                0.0,
                Vector3f {x: 0.0, y: 1.0, z: 0.0},
            )),
            material: Arc::new(DummyMaterial {}),
        };

        for y in -12..12 {
            for x in -12..12 {
                let primative = primative_original.clone();

                let x = (x as f32) / 10.0;
                let y = (y as f32) / 10.0;

                let ray = Ray {
                    a: Point3f  {x:   x, y:   y, z:  4.0},
                    b: Vector3f {x: 0.0, y: 0.0, z: -1.0},
                    time: 0.0,
                };
                let interaction_result = primative.collide(ray, 0.0, 10.0);

                // 1.0 - x * x - y * y
                if (x * x) / 4.0 + (y * y) / 4.0 > 1.0 {
                    // miss case
                    assert!(interaction_result.is_none(), "Expected failure from x:{}, y:{}", x, y);
                } else {
                    let interaction = interaction_result.expect(&format!("Expected success from x:{}, y:{}", x, y));

                    let z_squared = (4.0 - x * x - y * y).max(0.0);
                    let z_point = z_squared.sqrt();

                    // "Expected x:{}, y:{}, z:{} (z squared:{}) for point", x, y, z_point, z_squared
                    let expected = Point3f {x: x, y: y, z: z_point};
                    assert!(
                        approx_eq!(Point3f,  interaction.p, expected, ulps = 4),
                        "Point comparison failed - returned {:?} expected {:?} (z squared: {})",
                        interaction.p,
                        expected,
                        z_squared,
                    );

                    let expected_normal = Normal3f {x: x / 4.0, y: y / 4.0, z: z_point / 4.0};
                    assert!(
                        approx_eq!(
                            Normal3f,
                            interaction.normal.unit_vector(),
                            expected_normal.unit_vector()
                        ),
                        "Normal comparison failed - returned {:?} expected {:?}",
                        interaction.normal.unit_vector(),
                        expected_normal.unit_vector(),
                    );
                }
            }
        }
    }

    #[test]
    fn test_irregularly_scaled_sphere_normals() {
        let primative_original = Primative {
            shape: Arc::new(Sphere {radius: 1.0}),
            transform: Arc::new(SRTTransform::init(
                Vector3f {x: 0.0, y: 0.0, z: 0.0},
                4.0,
                0.5,
                1.0,
                0.0,
                Vector3f {x: 0.0, y: 1.0, z: 0.0},
            )),
            material: Arc::new(DummyMaterial {}),
        };

        for y in -6..7 {
            for x in -11..11 {
                let primative = primative_original.clone();

                let x = (x as f32) / 2.5;
                let y = (y as f32) / 10.0;

                let ray = Ray {
                    a: Point3f  {x:   x, y:   y, z:  4.0},
                    b: Vector3f {x: 0.0, y: 0.0, z: -1.0},
                    time: 0.0,
                };
                let interaction_result = primative.collide(ray, 0.0, 10.0);

                // 1.0 - x * x - y * y
                let a_sqred = 4.0_f32.powi(2);
                let b_sqred = 0.5_f32.powi(2);
                if (x * x) / a_sqred + (y * y) / b_sqred > 1.0 {
                    // miss case
                    assert!(interaction_result.is_none(), "Expected failure from x:{}, y:{}", x, y);
                } else {
                    let interaction = interaction_result.expect(&format!("Expected success from x:{}, y:{}", x, y));

                    let z_squared = (1.0 - x * x / a_sqred - y * y / b_sqred).max(0.0);
                    let z_point = z_squared.sqrt();

                    let expected = Point3f {x: x, y: y, z: z_point};
                    assert!(
                        approx_eq!(Point3f,  interaction.p, expected, ulps = 4),
                        "Point comparison failed - returned {:?} expected {:?} (z squared: {})",
                        interaction.p,
                        expected,
                        z_squared,
                    );

                    let expected_normal = Normal3f {x: x / a_sqred, y: y / b_sqred, z: z_point};
                    assert!(
                        approx_eq!(
                            Normal3f,
                            interaction.normal.unit_vector(),
                            expected_normal.unit_vector()
                        ),
                        "Normal comparison failed - returned {:?} expected {:?}",
                        interaction.normal.unit_vector(),
                        expected_normal.unit_vector(),
                    );
                }
            }
        }
    }

    // #[test]
    // fn test_moved_unit_sphere_normals() {
    //     unimplemented!();
    // }

    // #[test]
    // fn test_moved_regularly_scaled_sphere_normals() {
    //     unimplemented!();
    // }

    // #[test]
    // fn test_moved_irregularly_scaled_sphere_normals() {
    //     unimplemented!();
    // }
}
