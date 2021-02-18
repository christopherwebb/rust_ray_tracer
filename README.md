# Rust Ray-Tracer

A rust based implementation of Peter Shirley's [Ray Tracing In One Weekend](https://raytracing.github.io/) series

## How to run the demos:

![Final image of Ray Tracing In One Weekend. Contains three large sphere, surrounded by many smaller spheres. Some spheres are a matt colour, others are clear, while others are reflective](/images/image_030b.png)
To generate the final image of Ray Tracing In One Weekend:

```
Command goes here
```

The above will generate an image, ``, with the final result.

## What all the commands actually mean / do

### rust_ray_tracer

Takes in as input the scene json, and outputs json lines (one line per ray result).
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

At this point, make sure that the height and width given both to the 

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
