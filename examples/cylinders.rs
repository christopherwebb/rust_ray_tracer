use std::f32;

use clap::{Arg, App};
use serde::{Serialize};
use serde_json::{Result, Value};

extern crate rust_ray_tracing;

use rust_ray_tracing::core::{
    Point3f,
    Vector3f,
    Colour,
};
use rust_ray_tracing::camera::Camera;
use rust_ray_tracing::scene::{HitList, Scene};
use rust_ray_tracing::material::Material;
use rust_ray_tracing::shapes::base::Hitable;
use rust_ray_tracing::shapes::cylinder::Cylinder;
use rust_ray_tracing::shapes::sphere::{Sphere, MovingSphere};


fn cylinders(aspect : f32) -> Scene {
    let camera = Camera::create(
        Point3f  { x: 1.5, y: 1.0, z: 1.5 },
        Point3f  { x: 0.0, y: 0.0, z: 0.0 },
        Vector3f { x: 0.0, y: 1.0, z: 0.0 },
        90.0,
        aspect,
        0.1,
        10.0,
        0.0,
        0.0,
    );

    let hitlist = HitList {
        spheres: vec![],
        moving_spheres: vec![],
        cylinders: vec![
            Cylinder {
                centre: Point3f { x: 1.0, y: 0.0, z: -1.0},
                radius: 0.5,
                phi_max: 2.0 * f32::consts::PI,
                z_min: -0.25,
                z_max: 0.25,
                material: Material::make_lambertian(
                    Colour { r: 0.1, g: 0.2, b: 0.5 },
                )
            },
            Cylinder {
                centre: Point3f { x: 0.0, y: 0.0, z: -1.0},
                radius: 0.5,
                phi_max: 2.0 * f32::consts::PI,
                z_min: -1.0,
                z_max: 1.0,
                material: Material::make_metal(
                    Colour { r: 0.8, g: 0.6, b: 0.2 },
                    1.0,
                )
            },
            Cylinder {
                centre: Point3f { x: -1.0, y: 0.0, z: -1.0},
                radius: 0.5,
                phi_max: 2.0 * f32::consts::PI,
                z_min: -0.5,
                z_max: 0.5,
                material: Material::make_dielectric(1.5)
            },
            Cylinder {
                centre: Point3f { x: -1.0, y: 0.0, z: -1.0},
                radius: -0.45,
                phi_max: 2.0 * f32::consts::PI,
                z_min: -0.45,
                z_max: 0.45,
                material: Material::make_dielectric(1.5)
            },
        ],
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

    let example_scene = cylinders(aspect);

    let serialized = serde_json::to_string(&example_scene).unwrap();
    println!("{}", serialized);
}
