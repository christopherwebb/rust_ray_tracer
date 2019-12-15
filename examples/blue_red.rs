use std::f32;

use clap::{Arg, App};
use serde::{Serialize};
use serde_json::{Result, Value};

extern crate rust_ray_tracing;

use rust_ray_tracing::core::{
    Point3f, Vector3f
};
use rust_ray_tracing::camera::Camera;
use rust_ray_tracing::scene::{HitList, Scene};
use rust_ray_tracing::material::Material;
use rust_ray_tracing::vector::Vec3;
use rust_ray_tracing::shapes::base::Hitable;
use rust_ray_tracing::shapes::cylinder::Cylinder;
use rust_ray_tracing::shapes::sphere::{Sphere, MovingSphere};


fn blue_red_spheres(aspect : f32) -> Scene {
    let camera = Camera::create(
        Point3f  { x: -2.0, y: 2.0, z:  1.0 },
        Point3f  { x:  0.0, y: 0.0, z: -1.0 },
        Vector3f { x:  0.0, y: 1.0, z:  0.0 },
        45.0,
        aspect,
        0.1,
        10.0,
        0.0,
        0.0,
    );

    let sphere_radius: f32 = (f32::consts::PI / 4.0).cos();
    let hitlist = HitList {
        spheres: vec![
            Sphere {
                centre: Point3f { x: -sphere_radius, y: 0.0, z: -1.0},
                radius: sphere_radius,
                material: Material::make_lambertian(
                    Vec3 { e: [0.0, 0.0, 1.0]},
                )
            },
            Sphere {
                centre: Point3f { x: sphere_radius, y: 0.0, z: -1.0},
                radius: sphere_radius,
                material: Material::make_lambertian(
                    Vec3 { e: [1.0, 0.0, 0.0]},
                )
            },
        ],
        moving_spheres: vec![],
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

    let example_scene = blue_red_spheres(aspect);

    let serialized = serde_json::to_string(&example_scene).unwrap();
    println!("{}", serialized);
}
