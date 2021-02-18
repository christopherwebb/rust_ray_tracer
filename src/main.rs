use std::f32;
use std::io::{self, Read};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Instant;

use rand::thread_rng;
use rand::Rng;
use clap::{Arg, App};

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use console::style;

mod aabb;
mod bvh_tree;
mod camera;
mod core;
mod material2;
mod material;
mod primative;
mod ray;
mod render;
mod scene2;
mod scene;
mod scene_generator;
mod shapes;
mod vector;
mod textures;


use crate::core::{Point3f, Normal3f, Colour};
use crate::material::{
    Material,
    MaterialHit,
    HitRecord
};
use crate::ray::Ray;
use crate::render::RenderResult;
use crate::shapes::base::Hitable;
use crate::scene::HitList;

use crate::scene2::{Scene, calculate_colour};
use crate::scene_generator::three_sphere;


fn colour(ray : &Ray, world: &HitList, depth : i32) -> Colour {
    let mut hit_rec : HitRecord = HitRecord {
            t: 10000.0,
            p: Point3f { x: 0.0, y: 0.0, z: 0.0 },
            normal: Normal3f { x: 0.0, y: 0.0, z: 0.0 },
            material: Material::make_dummy_material(),
        };

    if world.hit(ray, 0.001, 100000.0, &mut hit_rec) {
        if depth >= 50 {
            return Colour { r: 0.0, g: 0.0, b: 0.0 };
        }

        let scatter_result : MaterialHit = hit_rec.material.scatter(&ray, &hit_rec);
        if !scatter_result.hit {
            return Colour { r: 0.0, g: 0.0, b: 0.0 };
        }

        return scatter_result.atten * colour(&scatter_result.ray_out, world, depth + 1);
    }

    let unit_dir = ray.direction().unit_vector();
    let t : f32 = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Colour { r: 1.0, g: 1.0, b: 1.0 } + t * Colour { r: 0.5, g: 0.7, b: 1.0 }
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

// fn main() {
//     let matches = App::new("Ray Tracer")
//        .version("0.1")
//        .about("Ray Tracer, written in rust, building off of Peter Shirley's Ray Tracing In One Weekend")
//        .author("Christopher Webb")
//        .arg(Arg::with_name("width")
//                .short("w")
//                .long("width")
//                .default_value("800")
//                .value_name("INT")
//                .help("Width of image")
//                .takes_value(true))
//        .arg(Arg::with_name("height")
//                .short("h")
//                .long("height")
//                .default_value("400")
//                .value_name("INT")
//                .help("Height of image")
//                .takes_value(true))
//         .arg(Arg::with_name("aa_samples")
//                .short("s")
//                .long("samples")
//                .default_value("100")
//                .value_name("INT")
//                .help("Number of samples per pixel")
//                .takes_value(true))
//         .arg(Arg::with_name("file")
//                 .short("f")
//                 .long("file")
//                 .value_name("FILE")
//                 .help("Load from file (not yet implemented)"))
//        .get_matches();

//     let started = Instant::now();

//     let NTHREADS = num_cpus::get() as u32;

//     let n_x : u32 = matches.value_of("width").unwrap().parse::<u32>().unwrap();
//     let n_y : u32 = matches.value_of("height").unwrap().parse::<u32>().unwrap();

//     eprintln!(
//         "{} Parsing scene...",
//         style("[1/2]").bold().dim(),
//     );
//     let mut buffer = String::new();
//     io::stdin().read_to_string(&mut buffer);
//     let input_scene: scene::Scene = serde_json::from_str(&buffer).unwrap();

//     let _aspect = (n_x as f32) / (n_y as f32);

//     let aa_samples : u32 = matches.value_of("aa_samples").unwrap().parse::<u32>().unwrap();

//     let arc_scene = Arc::new(input_scene);

//     eprintln!(
//         "{} Rendering scene...",
//         style("[2/2]").bold().dim(),
//     );
//     let pb = ProgressBar::new((n_y * n_x * aa_samples).into());
//     pb.set_style(
//         ProgressStyle::default_bar()
//         .template("{spinner:.green} {elapsed_precise} {bar:40.cyan/blue} {pos}/{len}")
//     );

//     for y_coord in (0..n_y).rev() {
//         for x_coord in 0..n_x {
//             let mut handles = vec![];
//             let (tx, rx) = mpsc::channel();

//             for thread in 0..NTHREADS {
//                 let arc_scene_n = Arc::clone(&arc_scene);
//                 let tx_n = mpsc::Sender::clone(&tx);

//                 let handle = thread::spawn(move || {
//                     let mut rng = thread_rng();

//                     let (from, to) = sample_range(thread, NTHREADS, aa_samples);
//                     for _ in from..to {
//                         let rand_x : f32 = rng.gen::<f64>() as f32;
//                         let rand_y : f32 = rng.gen::<f64>() as f32;

//                         let u: f32 = (rand_x + x_coord as f32) / n_x as f32;
//                         let v: f32 = (rand_y + y_coord as f32) / n_y as f32;

//                         let ray = &arc_scene_n.camera.get_ray(u, v);

//                         let colour_result = colour(&ray, &arc_scene_n.hitlist, 0);
//                         let result = RenderResult {
//                             x_coord: rand_x + x_coord as f32,
//                             y_coord: rand_y + y_coord as f32,
//                             time: ray.time,
//                             colour: colour_result,
//                         };
//                         tx_n.send(result).unwrap();
//                     }
//                 });
//                 handles.push(handle);
//             }

//             drop(tx);

//             for handle in handles {
//                 handle.join().unwrap();
//             }

//             for received in rx.iter() {
//                 pb.inc(1);
//                 let serialized = serde_json::to_string(&received).unwrap();
//                 println!("{}", serialized);
//             }
//         }
//     }

//     pb.finish_and_clear();
//     eprintln!(
//         "Scene done in {}!",
//         HumanDuration(started.elapsed()),
//     );
// }

fn main() {
    eprintln!("{} Initialising...", style("[0/2]").bold().dim());
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

    let started = Instant::now();

    let NTHREADS = num_cpus::get() as u32;
    eprintln!("{} Running with {} processor(s)...", style("[1/2]").bold().dim(), NTHREADS);

    let n_x : u32 = matches.value_of("width").unwrap().parse::<u32>().unwrap();
    let n_y : u32 = matches.value_of("height").unwrap().parse::<u32>().unwrap();

    // eprintln!(
    //     "{} Parsing scene...",
    //     style("[1/2]").bold().dim(),
    // );
    // let mut buffer = String::new();
    // io::stdin().read_to_string(&mut buffer);
    // let input_scene: scene::Scene = serde_json::from_str(&buffer).unwrap();

    eprintln!("{} Generating scene...", style("[1/2]").bold().dim());

    let scene = three_sphere(0.0, 1.0 / 30.0);

    let _aspect = (n_x as f32) / (n_y as f32);

    let aa_samples : u32 = matches.value_of("aa_samples").unwrap().parse::<u32>().unwrap();

    // let arc_scene = Arc::new(input_scene);
    let arc_scene = Arc::new(scene);

    eprintln!(
        "{} Rendering scene with a total of {} divisions...",
        style("[2/2]").bold().dim(),
        n_y * n_x * aa_samples,
    );
    let pb = ProgressBar::new((n_y * n_x * aa_samples).into());
    pb.set_style(
        ProgressStyle::default_bar()
        .template("{spinner:.green} {elapsed_precise} {bar:40.cyan/blue} {pos}/{len}")
    );

    // let mut colour_results: Vec<RenderResult> = vec![];
    // for y_coord in (0..n_y).rev() {
    //     for x_coord in 0..n_x {

    //         // let arc_scene_n = Arc::clone(&arc_scene);
    //         // let tx_n = mpsc::Sender::clone(&tx);

    //         // let handle = thread::spawn(move || {
    //             let mut rng = thread_rng();

                
    //             let (from, to) = sample_range(0, 1, aa_samples);
                

    //             for x_blah in from..to {
    //                 // eprintln!(
    //                 //     "Executing... y_coord: {} x_coord: {} sample {}",
    //                 //     y_coord,
    //                 //     x_coord,
    //                 //     x_blah,
    //                 // );
    //                 let rand_x : f32 = rng.gen::<f64>() as f32;
    //                 let rand_y : f32 = rng.gen::<f64>() as f32;

    //                 let u: f32 = (rand_x + x_coord as f32) / n_x as f32;
    //                 let v: f32 = (rand_y + y_coord as f32) / n_y as f32;

    //                 let ray = &scene.camera.get_ray(u, v);

    //                 // let colour_result = colour(&ray, &arc_scene_n.hitlist, 0);
    //                 let colour_result = calculate_colour(&scene, ray, 0);
    //                 let result = RenderResult {
    //                     x_coord: rand_x + x_coord as f32,
    //                     y_coord: rand_y + y_coord as f32,
    //                     time: ray.time,
    //                     colour: colour_result,
    //                 };
    //                 colour_results.push(result);
    //                 // tx_n.send(result).unwrap();
    //             }
    //         // });
    //         // handles.push(handle);


    //         // drop(tx);

    //         // for handle in handles {
    //         //     handle.join().unwrap();
    //         // }

            
    //     }
    // }

    // for received in colour_results.iter() {
    //     pb.inc(1);
    //     let serialized = serde_json::to_string(&received).unwrap();
    //     println!("{}", serialized);
    // }

    let mut ray_results : Vec<RenderResult> = vec![];
    for y_coord in (0..n_y).rev() {
        for x_coord in 0..n_x {
            let mut handles = vec![];
            let (tx, rx) = mpsc::channel();

            for thread in 0..NTHREADS {
                let arc_scene_n = Arc::clone(&arc_scene);
                let tx_n = mpsc::Sender::clone(&tx);

                let handle = thread::spawn(move || {
                    let mut rng = thread_rng();

                    let (from, to) = sample_range(thread, NTHREADS, aa_samples);
                    for _ in from..to {
                        let rand_x : f32 = rng.gen::<f64>() as f32;
                        let rand_y : f32 = rng.gen::<f64>() as f32;

                        let x_coord_precise = rand_x + x_coord as f32;
                        let y_coord_precise = rand_y + y_coord as f32;

                        let u: f32 = x_coord_precise / n_x as f32;
                        let v: f32 = y_coord_precise / n_y as f32;

                        let ray = &arc_scene_n.camera.get_ray(u, v);

                        let colour_result = calculate_colour(&arc_scene_n, ray, 0);
                        let result = RenderResult {
                            x_coord: x_coord_precise,
                            y_coord: y_coord_precise,
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

            let mut received_count = 0;
            for received in rx.iter() {
                received_count += 1;
                ray_results.push(received);
                let serialized = serde_json::to_string(&received).unwrap();
                println!("{}", serialized);
            }
            pb.inc(received_count);
        }
    }

    pb.finish_and_clear();
    eprintln!(
        "Scene done in {}!",
        HumanDuration(started.elapsed()),
    );

    // eprintln!()
}

#[cfg(test)]
mod main_tests {
    use rand::thread_rng;
    use rand::Rng;

    #[test]
    fn test_coordinate_bounding() {
        let mut rng = thread_rng();
        let initial_cord = 2000;
        for attempt in 0..100000 {
            let init_coord_f32 = initial_cord as f32;
            let rnd_f32 = rng.gen::<f64>() as f32;
            let f32_coord = init_coord_f32 + rnd_f32;
            let converted_back = f32_coord as usize;

            assert_eq!(converted_back, initial_cord, "Attempt {} converting {:.60} + {:.60} (becomes {:.60}) into int", attempt, init_coord_f32, rnd_f32, f32_coord)
        }
    }
}
