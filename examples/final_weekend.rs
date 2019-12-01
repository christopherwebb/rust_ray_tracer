use std::f32;

use clap::{Arg, App};
use rand::Rng;
use rand::thread_rng;
use serde::{Serialize};
use serde_json::{Result, Value};

extern crate rust_ray_tracing;

use rust_ray_tracing::camera::Camera;
use rust_ray_tracing::scene::{HitList, Scene};
use rust_ray_tracing::material::Material;
use rust_ray_tracing::vector::Vec3;
use rust_ray_tracing::shapes::base::Hitable;
use rust_ray_tracing::shapes::cylinder::Cylinder;
use rust_ray_tracing::shapes::sphere::{Sphere, MovingSphere};


fn final_weekend(aspect : f32) -> Scene {
	let mut rng = thread_rng();

	let look_from = Vec3 { e: [ 13.0, 2.0, 3.0]};
    let look_at = Vec3 { e: [ 0.0, 0.0, 0.0]};

    let camera = Camera::create(
        look_from,
        look_at,
        Vec3 { e: [ 0.0, 1.0,  0.0]},
        20.0,
        aspect,
        0.1,
        10.0,
        0.0,
        1.0,
    );

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

    let hitlist = HitList {
        spheres: sphere_list,
        moving_spheres: moving_sphere_list,
        cylinders: vec![],
    };

    Scene {
        hitlist: hitlist,
        camera: camera,
    }
}

fn main() {
    let matches = App::new("Ray Tracer")
       .version("0.1")
       .about("Ray Tracer, written in rust, building off of Peter Shirley's Ray Tracing In One Weekend")
       .author("Christopher Webb")
       .arg(Arg::with_name("aspect")
               .short("a")
               .long("aspect")
               .default_value("2.0")
               .value_name("FLOAT")
               .help("Aspect ratio")
               .takes_value(true))
       .get_matches();

    let aspect : f32 = matches
        .value_of("aspect")
        .unwrap()
        .parse::<f32>()
        .unwrap();

    let example_scene = final_weekend(aspect);

    let serialized = serde_json::to_string(&example_scene).unwrap();
    println!("{}", serialized);
}
