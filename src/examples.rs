use std::f32;

use rand::Rng;
use rand::rngs::ThreadRng;

use crate::material::Material;

use crate::vector::Vec3;

use crate::world::{
    HitList,
    Sphere,
    MovingSphere,
    Cylinder,
};

use crate::camera::Camera;

impl HitList {
    pub fn three_spheres_on_world() -> HitList {
        HitList {
            spheres: vec![
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
            ],
            moving_spheres: vec![],
            cylinders: vec![],
        }
    }
    pub fn blue_red_spheres() -> HitList {
        let sphere_radius: f32 = (f32::consts::PI / 4.0).cos();
        HitList {
            spheres: vec![
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
            ],
            moving_spheres: vec![],
            cylinders: vec![],
        }
    }
    pub fn cylinders() -> HitList {
        HitList {
            spheres: vec![],
            moving_spheres: vec![],
            cylinders: vec![
                Cylinder {
                    centre: Vec3 { e: [1.0, 0.0, -1.0]},
                    radius: 0.5,
                    phi_max: 2.0 * f32::consts::PI,
                    zMin: -0.25,
                    zMax: 0.25,
                    material: Material::make_lambertian(
                        Vec3 { e: [0.1, 0.2, 0.5]},
                    )
                },
                Cylinder {
                    centre: Vec3 { e: [0.0, 0.0, -1.0]},
                    radius: 0.5,
                    phi_max: 2.0 * f32::consts::PI,
                    zMin: -1.0,
                    zMax: 1.0,
                    material: Material::make_metal(
                        Vec3 { e: [0.8, 0.6, 0.2]},
                        1.0,
                    )
                },
                Cylinder {
                    centre: Vec3 { e: [-1.0, 0.0, -1.0]},
                    radius: 0.5,
                    phi_max: 2.0 * f32::consts::PI,
                    zMin: -0.5,
                    zMax: 0.5,
                    material: Material::make_dielectric(1.5)
                },
                Cylinder {
                    centre: Vec3 { e: [-1.0, 0.0, -1.0]},
                    radius: -0.45,
                    phi_max: 2.0 * f32::consts::PI,
                    zMin: -0.45,
                    zMax: 0.45,
                    material: Material::make_dielectric(1.5)
                },
            ],
        }
    }
    pub fn random_world(rng: &mut ThreadRng) -> HitList {
        let small_radius = 0.2;
        let large_radius = 1.0;
        let mut sphere_list = vec![];
        let mut moving_sphere_list = vec![];

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
                    match chosen_mat {
                        x if x < 0.8 => moving_sphere_list.push(MovingSphere {
                            centre0: centre,
                            centre1: centre + Vec3{ e: [
                                0.0, 0.5 * rng.gen::<f64>() as f32, 0.0
                            ]},
                            time0: 0.0,
                            time1: 1.0,
                            radius: 0.2,
                            material: Material::make_lambertian(Vec3 { e: [
                                rng.gen::<f64>() as f32 * rng.gen::<f64>() as f32,
                                rng.gen::<f64>() as f32 * rng.gen::<f64>() as f32,
                                rng.gen::<f64>() as f32 * rng.gen::<f64>() as f32,
                            ]}),
                        }),
                        x if x < 0.95 => sphere_list.push(Sphere {
                            centre: centre,
                            radius: 0.2,
                            material: Material::make_metal(Vec3 { e: [
                                0.5 * (1.0 + rng.gen::<f64>() as f32),
                                0.5 * (1.0 + rng.gen::<f64>() as f32),
                                0.5 * (1.0 + rng.gen::<f64>() as f32),
                            ]}, 0.5 * rng.gen::<f64>() as f32),
                        }),
                        _ => sphere_list.push(Sphere {
                            centre: centre,
                            radius: 0.2,
                            material: Material::make_dielectric(1.5),
                        })
                    }
                }
            }
        }

        HitList {
            spheres: sphere_list,
            moving_spheres: moving_sphere_list,
            cylinders: vec![],
        }
    }
}

pub fn generate_example(example_name: String, rng: &mut ThreadRng, aspect: f32) -> (HitList, Camera) {
    match example_name.as_ref() {
        "3balls" => {(
            HitList::three_spheres_on_world(),
            Camera::create(
                Vec3 { e: [ 0.0, 0.0,  0.0]},
                Vec3 { e: [ 0.0, 0.0, -1.0]},
                Vec3 { e: [ 0.0, 1.0,  0.0]},
                90.0,
                aspect,
                0.1,
                10.0,
                0.0,
                0.0,
            )
        )},
        "cylinders" => {(
            HitList::cylinders(),
            Camera::create(
                Vec3 { e: [ 1.5, 1.0,  1.5]},
                Vec3 { e: [ 0.0, 0.0, 0.0]},
                Vec3 { e: [ 0.0, 1.0,  0.0]},
                90.0,
                aspect,
                0.1,
                10.0,
                0.0,
                0.0,
            )
        )},
        "blue_red" => {(
            HitList::blue_red_spheres(),
            Camera::create(
                Vec3 { e: [-2.0, 2.0,  1.0]},
                Vec3 { e: [ 0.0, 0.0, -1.0]},
                Vec3 { e: [ 0.0, 1.0,  0.0]},
                45.0,
                aspect,
                0.1,
                10.0,
                0.0,
                0.0,
            ),
        )},
        "final_weekend" => {
            let look_from = Vec3 { e: [ 13.0, 2.0, 3.0]};
            let look_at = Vec3 { e: [ 0.0, 0.0, 0.0]};
            (
                HitList::random_world(rng),
                Camera::create(
                    look_from,
                    look_at,
                    Vec3 { e: [ 0.0, 1.0,  0.0]},
                    20.0,
                    aspect,
                    0.1,
                    10.0,
                    0.0,
                    1.0,
                ),
            )
        },
        _ => {(
            HitList{
                spheres: vec![],
                moving_spheres: vec![],
                cylinders: vec![],
            },
            Camera::create(
                Vec3 { e: [ 0.0, 0.0,  0.0]},
                Vec3 { e: [ 0.0, 0.0, -1.0]},
                Vec3 { e: [ 0.0, 1.0,  0.0]},
                90.0,
                aspect,
                0.1,
                10.0,
                0.0,
                0.0,
            ),
        )}
    }
}
