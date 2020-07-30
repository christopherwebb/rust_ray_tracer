use serde::{Deserialize, Serialize};

use crate::core::{
    Normal3f,
    Point3f,
    Matrix4x4f,
    dot_vv,
};

use crate::ray::Ray;
use crate::shapes::base::{Interaction, ShapeTrait};


struct Sphere {
    pub radius: f32,
}

impl ShapeTrait for Sphere {
    fn collide(
        &self,
        ray: Ray,
        object_to_world: Matrix4x4f,
        world_to_object: Matrix4x4f,
        t_min: f32,
        t_max: f32,
    ) -> Option<Interaction> {
        let t_ray = world_to_object * ray;
        let ray_o = t_ray.origin();
        let ray_d = t_ray.direction();

        // let direction_dot = ray.direction();
        let a: f32 = dot_vv(&ray_d, &ray_d);
        let b: f32 = ray_o.x * ray_d.x + ray_o.y * ray_d.y + ray_o.z * ray_d.z;
        let c: f32 = ray_o.x * ray_o.x + ray_o.y * ray_o.y + ray_o.z * ray_o.z - self.radius * self.radius;

        let discriminant : f32 = b * b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let temp: f32 = (-b - (b * b - a * c).sqrt()) / a;
        if temp > t_min && temp < t_max {
            let interaction_point =  t_ray.point_at_parameter(temp);
            return Some(Interaction {
                t: temp,
                p: object_to_world * interaction_point,
                normal: object_to_world * Normal3f::from(&interaction_point - Point3f {x: 0.0, y: 0.0, z: 0.0}) / self.radius,
            });
        }

        let temp2: f32 = (-b + (b * b - a * c).sqrt()) / a;
        if temp2 > t_min && temp2 < t_max {
            let interaction_point =  t_ray.point_at_parameter(temp2);
            return Some(Interaction {
                t: temp2,
                p: object_to_world * interaction_point,
                normal: object_to_world * Normal3f::from(&interaction_point - Point3f {x: 0.0, y: 0.0, z: 0.0}) / self.radius,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        // Normal3f,
        Point3f,
        Vector3f,
        Matrix4x4f,
        // dot_vv,
        indentity,
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

        match result_option {
            Some(result) => assert_eq!(result.t, 4.0),
            None => assert!(false),
        }
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
            0.0,
            20.0,
        );

        match result_option {
            Some(result) => assert_eq!(result.t, 5.0),
            None => assert!(false),
        }
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

        match result_option {
            Some(result) => assert!(false),
            None => (),
        }
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

    // Scenario: Intersect sets the object on the intersection
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0].object = s
    //     And xs[1].object = s
}
