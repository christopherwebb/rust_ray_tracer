use std::f32;
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
};

mod examples;

fn colour(ray : &Ray, world: &Hitable, depth : i32) -> Vec3 {
    let mut hit_rec : HitRecord = HitRecord {
            t: 10000.0,
            p: Vec3 { e: [0.0, 0.0, 0.0]},
            normal: Vec3 { e: [0.0, 0.0, 0.0]},
            material: Material::make_dummy_material(),
        };

    if world.hit(ray, 0.001, 10000.0, &mut hit_rec) {
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
        .arg(Arg::with_name("time")
               .short("t")
               .long("time")
               .default_value("0")
               .value_name("INT")
               .help("Time of shutter open")
               .takes_value(true))
        .arg(Arg::with_name("shutter_length")
               .long("time_length")
               .default_value("0")
               .value_name("INT")
               .help("Length of time shutter is open")
               .takes_value(true))
        .arg(Arg::with_name("example")
               .long("example")
               .value_name("EXAMPLE")
               .help("generate builtin example")
               .possible_values(&["3balls", "blue_red", "final_weekend"])
               .takes_value(true)
               .conflicts_with("file"))
        .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Load from file (not yet implemented)"))
       .get_matches();

    let n_x : i32 = matches.value_of("width").unwrap().parse::<i32>().unwrap();
    let n_y : i32 = matches.value_of("height").unwrap().parse::<i32>().unwrap();
    let aspect = (n_x as f32) / (n_y as f32);


    let time_start = matches.value_of("time").unwrap().parse::<f32>().unwrap();
    let time_length = matches.value_of("shutter_length").unwrap().parse::<f32>().unwrap();

    let aa_samples : i32 = matches.value_of("aa_samples").unwrap().parse::<i32>().unwrap();
    let aa_division : f32 = aa_samples as f32;

    let mut rng = thread_rng();

    let example = matches.value_of("example").unwrap().to_string();
    let (world, cam) = examples::generate_example(example, &mut rng, aspect);

    println!("P3\n{} {}\n255", n_x, n_y);
    for y_coord in (0..n_y).rev() {
        for x_coord in 0..n_x {
            let mut col_sum = Vec3 { e: [0.0, 0.0, 0.0]};
            for _aa_iter in 0..aa_samples {
                let rand_x : f32 = rng.gen::<f64>() as f32;
                let rand_y : f32 = rng.gen::<f64>() as f32;

                let u: f32 = (rand_x + x_coord as f32) / n_x as f32;
                let v: f32 = (rand_y + y_coord as f32) / n_y as f32;

                let ray = &cam.get_ray(u, v);

                col_sum += colour(&ray, &world, 0);
            }
            let col : Vec3 = col_sum / aa_division;

            let ir = (255.99 * col.r()) as u64;
            let ig = (255.99 * col.g()) as u64;
            let ib = (255.99 * col.b()) as u64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
