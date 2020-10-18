use std::io::{self, Read};
use std::io::prelude::*;

use clap::{Arg, App};

extern crate rust_ray_tracing;

use rust_ray_tracing::render::RenderResult;
use rust_ray_tracing::core::Colour;


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
       .get_matches();
    let n_x : u32 = matches.value_of("width").unwrap().parse::<u32>().unwrap();
    let n_y : u32 = matches.value_of("height").unwrap().parse::<u32>().unwrap();

    let mut sorted_results : Vec<Vec<Vec<RenderResult>>> = vec![vec![vec![]; n_x as usize]; n_y as usize];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // println!("{}", line.unwrap());
        let input_ray: RenderResult = serde_json::from_str(&line.unwrap()).unwrap();
        sorted_results[(n_y as f32 - (input_ray.y_coord + 1.0)) as usize][input_ray.x_coord as usize].push(input_ray);
    }
    // let mut buffer = String::new();
 //    io::stdin().read_to_string(&mut buffer);
 //    let input_rays: RenderResult = serde_json::from_str(&buffer).unwrap();

    
    // for result in &results {
        
    // }

    println!("P3\n{} {}\n255", n_x, n_y);
    for y_results in sorted_results {
        for pixel_results in y_results {
            let mut col_sum = Colour { r: 0.0, g: 0.0, b: 0.0 };
            for result in &pixel_results {
                col_sum += result.colour;
            }

            let col = match pixel_results.len() {
              0 => col_sum,
              _ => col_sum / pixel_results.len() as f32,
            };

            let ir = (255.99 * col.r) as u64;
            let ig = (255.99 * col.g) as u64;
            let ib = (255.99 * col.b) as u64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
