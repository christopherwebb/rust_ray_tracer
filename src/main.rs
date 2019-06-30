use std::io::{self, Write};

fn main() {
    let nX = 200;
    let nY = 200;

    println!("P3\n{} {}\n255", nX, nY);
    for yCoord in (0..nY).rev() {
        for xCoord in (0..nX) {
            let r = xCoord as f32 / nX as f32;
            let g = yCoord as f32 / nY as f32;
            let b = 0.2;

            let ir = (255.99 * r) as u64;
            let ig = (255.99 * g) as u64;
            let ib = (255.99 * b) as u64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
