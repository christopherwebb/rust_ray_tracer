# Rust Ray-Tracer

A rust based implementation of Peter Shirley's [Ray Tracing In One Weekend](https://raytracing.github.io/) series

## How to run the demos:

![Final image of Ray Tracing In One Weekend. Contains three large sphere, surrounded by many smaller spheres. Some spheres are a matt colour, others are clear, while others are reflective](/examples_images/final_weekend.png)
To generate the final image of Ray Tracing In One Weekend:

```
cargo run --example final_weekend 2>/dev/null \
 | cargo run --bin rust_ray_tracing -- -w 2000 -h 1000 -s 100 \
 | cargo run --bin rust_ray_assemble -- -w 2000 -h 1000 \
 > final_image.ppm
```

The above will generate an image, `final_image.ppm`, with the final result.

## But what do all those commands actually mean / do

### The example generator

`cargo run --example final_weekend` generates the JSON scene. This command can, of course, be outputed into a json file, and that file fed into the ray tracer.

### rust_ray_tracer

The `cargo run --bin rust_ray_tracing -w 2000 -h 1000 -s 100` command runs the ray tracer to generate a 2000x1000 image, with 100 samples per pixel.

The actual ray tracer. Takes in as input the scene json, and outputs json lines (one line per ray result). Takes an awful long time, and uses stderr to render a progress bar.

```
USAGE:
    rust_ray_tracing [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --samples <INT>    Number of samples per pixel [default: 100]
    -f, --file <FILE>      Load from file (not yet implemented)
    -h, --height <INT>     Height of image [default: 400]
    -w, --width <INT>      Width of image [default: 800]
```

### rust_ray_assemble

The `cargo run --bin rust_ray_assemble -w 2000 -h 1000` command takes in the rays produced by the ray tracer, and outputs a 2000x1000 image in PPM format.

Receives the json line results from the tracer, and averages the results out to produce the final PPM format image. Waits until the end of the input (EOF) before it begins the averaging process.

```
USAGE:
    rust_ray_assemble [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --height <INT>    Height of image [default: 400]
    -w, --width <INT>     Width of image [default: 800]
```

At this point, make sure that the height and width given to the tracer and assembler are the same.

## Exploring the source code

```
./src
  /bin - The ray assembler lives here
  /core - Vectors, points, matricies and transforms all live in the core module
  /shapes - Geometric shapes; spheres, cylinders and cones
  /textures - Textures for rendering onto the primative
  
  /aabb.rs - Simple AABB cube for acceleration
  /bvh_tree.rs BVH Tree acceleration structure
  /camera.rs Camera object
  /lib.rs Exposes the renderer as a library
  /main.rs rust_ray_tracer binary
  /material.rs Material used to calculate the effect of a ray that has intersected an object
  /primative.rs - Primative value used to hold the data used to render an object; the shape, the transforms and the material
  /ray.rs - Struct that holds the ray data, including the origin of the ray and the vector of where its going
  /scene.rs - Holds scene data, and is called into to find ray collisions
  /vector.rs - Old 3 dimensional vector. Deprecated in favour of the core module
```

## Current state

At the moment there's a lot going on, as I refactor the codebase to support transforms and materials in a parent object (the Primative), separate from the Shape object
