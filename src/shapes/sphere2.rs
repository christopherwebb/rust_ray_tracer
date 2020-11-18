use serde::{Deserialize, Serialize};

use crate::aabb::AABB;
use crate::core::{
    Normal3f,
    Point3f,
    Matrix4x4f,
    dot_vv,
    dot_vn,
};

use crate::ray::Ray;
use crate::shapes::base::{Interaction, ShapeTrait};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Sphere {
    pub radius: f32,
}

impl ShapeTrait for Sphere {
    fn collide(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<Interaction> {
        let t_ray = ray;
        let ray_o = t_ray.origin();
        let ray_d = t_ray.direction();

        // let direction_dot = ray.direction();
        let a: f32 = dot_vv(&ray_d, &ray_d);
        let half_b: f32 = ray_o.x * ray_d.x + ray_o.y * ray_d.y + ray_o.z * ray_d.z;
        // let half_b: f32 = dot_vn(&ray_o, &ray_d);
        let c: f32 = ray_o.x * ray_o.x + ray_o.y * ray_o.y + ray_o.z * ray_o.z - self.radius * self.radius;
        // let c: f32 = ray_o.squared_length() - self.radius * self.radius;

        let discriminant : f32 = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let root = discriminant.sqrt();

        let temp: f32 = (-half_b - root) / a;
        if temp > t_min && temp < t_max {
            let interaction_point =  t_ray.point_at_parameter(temp);
            return Some(Interaction {
                t: temp,
                p: interaction_point,
                normal: Normal3f::from(&interaction_point - Point3f {x: 0.0, y: 0.0, z: 0.0}),
            });
        }

        let temp2: f32 = (-half_b + root) / a;
        if temp2 > t_min && temp2 < t_max {
            let interaction_point =  t_ray.point_at_parameter(temp2);
            return Some(Interaction {
                t: temp2,
                p: interaction_point,
                normal: Normal3f::from(&interaction_point - Point3f {x: 0.0, y: 0.0, z: 0.0}),
            });
        }

        None
    }

    fn bounding_box(
        &self,
        time_0: f32,
        time_1: f32,
    ) -> Option<AABB> {
        Some(AABB {
            minimum: Point3f {
                x: -self.radius,
                y: -self.radius,
                z: -self.radius,
            },
            maximum: Point3f {
                x:  self.radius,
                y:  self.radius,
                z:  self.radius,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use std::f32;
    use crate::core::{
        Point3f,
        Vector3f,
        indentity,
        gen_scale,
        gen_translate,
        gen_rotate,
        gen_rotate_x,
        gen_rotate_y,
        gen_rotate_z,
    };

    use crate::ray::Ray;

    use crate::shapes::base::ShapeTrait;
    use crate::shapes::sphere2::Sphere;

    // Scenario: A ray intersects a sphere at two points
    // Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //   And s ← sphere()
    // When xs ← intersect(s, r)
    // Then xs.count = 2
    //   And xs[0] = 4.0
    //   And xs[1] = 6.0
    #[test]
    fn simple_intersection() {
        let sphere = Sphere {radius: 1.0};

        let result_option = sphere.collide(
            Ray {
                a: Point3f {
                    x:  0.0,
                    y:  0.0,
                    z: -5.0,
                },
                b: Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                time: 0.0,
            },
            indentity(),
            indentity(),
            0.0,
            20.0,
        );

        assert_eq!(result_option.unwrap().t, 4.0);
    }

    // Scenario: A ray intersects a sphere at a tangent
    //   Given r ← ray(point(0, 1, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0] = 5.0
    //     And xs[1] = 5.0
    #[test]
    fn tangent_intersection() {
        let sphere = Sphere {radius: 1.0};

        let result_option = sphere.collide(
            Ray {
                a: Point3f {
                    x:  0.0,
                    y:  1.0,
                    z: -5.0,
                },
                b: Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                time: 0.0,
            },
            indentity(),
            indentity(),
            4.999,
            5.001,
        );

        assert_eq!(result_option.unwrap().t, 5.0);
    }

    // Scenario: A ray misses a sphere
    //   Given r ← ray(point(0, 2, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 0
    #[test]
    fn missed_intersection() {
        let sphere = Sphere {radius: 1.0};

        let result_option = sphere.collide(
            Ray {
                a: Point3f {
                    x:  0.0,
                    y:  2.0,
                    z: -5.0,
                },
                b: Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                time: 0.0,
            },
            indentity(),
            indentity(),
            0.0,
            20.0,
        );

        assert_eq!(result_option.is_none(), true);
    }

    // Scenario: A ray originates inside a sphere
    //   Given r ← ray(point(0, 0, 0), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0] = -1.0
    //     And xs[1] = 1.0
    #[test]
    fn internal_intersection() {
        let sphere = Sphere {radius: 1.0};

        let result_option = sphere.collide(
            Ray {
                a: Point3f {
                    x:  0.0,
                    y:  0.0,
                    z:  0.0,
                },
                b: Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                time: 0.0,
            },
            indentity(),
            indentity(),
            0.0,
            20.0,
        );

        match result_option {
            Some(result) => assert_eq!(result.t, 1.0),
            None => assert!(false),
        }
    }

    // Scenario: A sphere is behind a ray
    //   Given r ← ray(point(0, 0, 5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0] = -6.0
    //     And xs[1] = -4.0

    // Scenario: Intersecting a scaled sphere with a ray
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When set_transform(s, scaling(2, 2, 2))
    //     And xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0].t = 3
    //     And xs[1].t = 7
    #[test]
    fn scaled_sphere() {
        let sphere = Sphere {radius: 1.0};
        let transform = gen_scale(6.0, 6.0, 6.0);

        let camera_a = Point3f {
                    x: -25.0,
                    y: -25.0,
                    z: -25.0,
                };
        let look_at = Point3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                } - camera_a;

        let ray = Ray {
                a: camera_a,
                b: look_at,
                time: 0.0,
            };

        let val_1 = (43.301270189221932 - 6.0) / look_at.length();
        let original_t_min = 0.99 * val_1;
        let original_t_max = 1.01 * val_1;

        // let updated_b = transform.m_inv * ray.b;

        let result_option = sphere.collide(
            ray,
            transform.m,
            transform.m_inv,
            original_t_min,
            original_t_max,
        );

        result_option.unwrap();
    }

    // Scenario: Intersecting a translated sphere with a ray
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When set_transform(s, translation(5, 0, 0))
    //     And xs ← intersect(s, r)
    //   Then xs.count = 0
    #[test]
    fn translated_missed_sphere() {
        let sphere = Sphere {radius: 1.0};
        let transform = gen_translate(Vector3f{
            x: 5.0,
            y: 0.0,
            z: 0.0,
        });

        let result_option = sphere.collide(
            Ray {
                a: Point3f {
                    x:  0.0,
                    y:  0.0,
                    z: -5.0,
                },
                b: Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                time: 0.0,
            },
            transform.m,
            transform.m_inv,
            0.0,
            20.0,
        );

        assert_eq!(result_option.is_none(), true);
    }

    // Scenario: Intersecting a translated sphere with a ray
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When set_transform(s, translation(5, 0, 0))
    //     And xs ← intersect(s, r)
    //   Then xs.count = 0
    #[test]
    fn translated_sphere() {
        let sphere = Sphere {radius: 1.0};
        let transform = gen_translate(Vector3f{
            x: 5.0,
            y: 0.0,
            z: 0.0,
        });

        let result_option = sphere.collide(
            Ray {
                a: Point3f {
                    x:  -5.0,
                    y:   0.0,
                    z:   0.0,
                },
                b: Vector3f {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                time: 0.0,
            },
            transform.m,
            transform.m_inv,
            0.0,
            20.0,
        );

        match result_option {
            Some(result) => assert_eq!(result.t, 9.0),
            None => assert!(false),
        }
    }

    #[test]
    fn rotated_sphere() {
        let sphere = Sphere {radius: 1.0};

        let transform = gen_rotate_x(0.0) * gen_rotate_y(0.0) * gen_rotate_z(0.0);

        let result_option = sphere.collide(
            Ray {
                a: Point3f {
                    x:  0.0,
                    y:  0.0,
                    z: -5.0,
                },
                b: Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                time: 0.0,
            },
            transform.m,
            transform.m_inv,
            0.0,
            20.0,
        );

        match result_option {
            Some(result) => assert_eq!(result.t, 4.0),
            None => assert!(false),
        }
    }

    #[test]
    fn moved_then_rotated_sphere() {
        let sphere = Sphere {radius: 1.0};

        let translation = gen_translate(Vector3f{
            x: 25.0,
            y: 0.0,
            z: 0.0,
        });
        let rotation = gen_rotate_z(0.5 * f32::consts::PI);
        let transform = rotation * translation;

        let result_option = sphere.collide(
            Ray {
                a: Point3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                b: Vector3f {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                time: 0.0,
            },
            transform.m,
            transform.m_inv,
            0.0,
            50.0,
        );

        assert_eq!(result_option.unwrap().t, 24.0);
    }

    #[test]
    fn moved_then_axis_rotated_sphere() {
        let sphere = Sphere {radius: 1.0};

        let translation = gen_translate(Vector3f{
            x: 25.0,
            y: 0.0,
            z: 0.0,
        });
        let rotation = gen_rotate(0.5 * f32::consts::PI, Vector3f {x: 1.0, y: 1.0, z: 0.0});
        let transform = rotation * translation;

        let x_pos = 12.5f32;
        let y_pos = 12.5f32;
        let z_pos = -(2.0f32 * x_pos.powi(2)).sqrt();
        let look_at = Vector3f {
                    x: x_pos,
                    y: y_pos,
                    z: z_pos,
                };

        let look_at_l = look_at.length();
        let t_val = (look_at_l - 1.0) / look_at_l;

        let centre = transform.m * Point3f{x: 0.0, y: 0.0, z: 0.0};
        println!("Hypothetical centre: x:{0}, y:{1}, z:{2}", centre.x, centre.y, centre.z);
        println!("We are aiming at: x:{0}, y:{1}, z:{2}", x_pos, y_pos, z_pos);

        let result_option = sphere.collide(
            Ray {
                a: Point3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                b: look_at,
                time: 0.0,
            },
            transform.m,
            transform.m_inv,
            t_val * 0.99,
            t_val * 1.01,
        );

        result_option.unwrap();
    }

    // Scenario: The normal on a sphere at a nonaxial point
    //   Given s ← sphere()
    //   When n ← normal_at(s, point(√3/3, √3/3, √3/3))
    //   Then n = vector(√3/3, √3/3, √3/3)

    //   Scenario: Computing the normal on a translated sphere
    //   Given s ← sphere()
    //     And set_transform(s, translation(0, 1, 0))
    //   When n ← normal_at(s, point(0, 1.70711, -0.70711))
    //   Then n = vector(0, 0.70711, -0.70711)

    // Scenario: Computing the normal on a transformed sphere
    //   Given s ← sphere()
    //     And m ← scaling(1, 0.5, 1) * rotation_z(π/5)
    //     And set_transform(s, m)
    //   When n ← normal_at(s, point(0, √2/2, -√2/2))
    //   Then n = vector(0, 0.97014, -0.24254)
}
