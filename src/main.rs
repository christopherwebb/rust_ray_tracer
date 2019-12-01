use std::cmp;
use std::f32;
use std::io::{self, Read};
use std::sync::{Arc, mpsc};
use std::thread;

use rand::thread_rng;
use rand::Rng;
use clap::{Arg, App};

mod material;
use crate::material::{
    Material,
    MaterialHit,
    HitRecord
};

mod ray;
use crate::ray::Ray;

mod vector;
use crate::vector::{
    Vec3,
    unit_vector,
};

mod camera;

mod world;
use crate::world::{
    Hitable,
    HitList,
};

mod scene;

mod render;
use crate::render::RenderResult;


fn colour(ray : &Ray, world: &HitList, depth : i32) -> Vec3 {
    let mut hit_rec : HitRecord = HitRecord {
            t: 10000.0,
            p: Vec3 { e: [0.0, 0.0, 0.0]},
            normal: Vec3 { e: [0.0, 0.0, 0.0]},
            material: Material::make_dummy_material(),
        };

    if world.hit(ray, 0.001, 100000.0, &mut hit_rec) {
        if depth >= 50 {
            return Vec3 { e: [0.0, 0.0, 0.0]};
        }

        let scatter_result : MaterialHit = hit_rec.material.scatter(&ray, &hit_rec);
        if !scatter_result.hit {
            return Vec3 { e: [0.0, 0.0, 0.0]};
        }

        return scatter_result.atten * colour(&scatter_result.ray_out, world, depth + 1);
    }

    let dir : Vec3 = ray.direction();
    let unit_dir : Vec3 = unit_vector(&dir);
    let t : f32 = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vec3 { e: [1.0, 1.0, 1.0]} + t * Vec3 { e: [0.5, 0.7, 1.0]}
}

fn sample_range(
    thread_number: u32,
    total_threads : u32,
    total_samples: u32
) -> (u32, u32) {
    let from = thread_number * total_samples / total_threads;
    let to = (thread_number + 1) * total_samples / total_threads;
    (from, to)
}

fn main() {
    let matches = App::new("Ray Tracer")
       .version("0.1")
       .about("Ray Tracer, written in rust, building off of Peter Shirley's Ray Tracing In One Weekend")
       .author("Christopher Webb")
       .arg(Arg::with_name("width")
               .short("w")
               .long("width")
               .default_value("800")
               .value_name("INT")
               .help("Width of image")
               .takes_value(true))
       .arg(Arg::with_name("height")
               .short("h")
               .long("height")
               .default_value("400")
               .value_name("INT")
               .help("Height of image")
               .takes_value(true))
        .arg(Arg::with_name("aa_samples")
               .short("s")
               .long("samples")
               .default_value("100")
               .value_name("INT")
               .help("Number of samples per pixel")
               .takes_value(true))
        .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Load from file (not yet implemented)"))
       .get_matches();

    let NTHREADS = num_cpus::get() as u32;

    let n_x : u32 = matches.value_of("width").unwrap().parse::<u32>().unwrap();
    let n_y : u32 = matches.value_of("height").unwrap().parse::<u32>().unwrap();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);
    let input_scene: scene::Scene = serde_json::from_str(&buffer).unwrap();

    let aspect = (n_x as f32) / (n_y as f32);

    let aa_samples : u32 = matches.value_of("aa_samples").unwrap().parse::<u32>().unwrap();
    let aa_division : f32 = aa_samples as f32;

    let arc_scene = Arc::new(input_scene);

    let mut results : Vec::<RenderResult> = vec![];
    for y_coord in (0..n_y).rev() {
        for x_coord in 0..n_x {
            let mut handles = vec![];
            let (tx, rx) = mpsc::channel();

            for thread in 0..NTHREADS {
                let arc_scene_n = Arc::clone(&arc_scene);
                let tx_n = mpsc::Sender::clone(&tx);

                let handle = thread::spawn(move || {
                    let mut rng = thread_rng();

                    let mut col_sum = Vec3 { e: [0.0, 0.0, 0.0]};

                    let (from, to) = sample_range(thread, NTHREADS, aa_samples);
                    for _ in from..to {
                        let rand_x : f32 = rng.gen::<f64>() as f32;
                        let rand_y : f32 = rng.gen::<f64>() as f32;

                        let u: f32 = (rand_x + x_coord as f32) / n_x as f32;
                        let v: f32 = (rand_y + y_coord as f32) / n_y as f32;

                        let ray = &arc_scene_n.camera.get_ray(u, v);

                        let colour_result = colour(&ray, &arc_scene_n.hitlist, 0);
                        let result = RenderResult {
                            x_coord: rand_x + x_coord as f32,
                            y_coord: rand_y + y_coord as f32,
                            time: ray.time,
                            colour: colour_result,
                        };
                        tx_n.send(result).unwrap();
                    }
                });
                handles.push(handle);
            }

            drop(tx);

            for handle in handles {
                handle.join().unwrap();
            }

            for received in rx.iter() {
                let serialized = serde_json::to_string(&received).unwrap();
                println!("{}", serialized);
            }
        }
    }
}
