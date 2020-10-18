// use std::rc::Rc;
use std::sync::Arc;

use crate::shapes::sphere2::Sphere;
use crate::primative::{Primative, TransformTrait};
use crate::{camera::Camera, core::{
    Vector3f,
    Point3f,
    Normal3f,
    Transform,
    gen_translate,
    gen_scale,
    gen_rotate_x,
    gen_rotate_y,
    gen_rotate_z,
    gen_rotate,
}, scene2::Scene};

struct SRTTransform {
    scale: Transform,
    rotate: Transform,
    translate: Transform,
}

impl SRTTransform {
    fn init(
        delta: Vector3f,
        scale_x: f32,
        scale_y: f32,
        scale_z: f32,
        theta: f32,
        axis: Vector3f,
    ) -> SRTTransform {
        SRTTransform {
            scale: gen_scale(scale_x, scale_y, scale_z),
            rotate: gen_rotate(theta, axis),
            translate: gen_translate(delta),
        }
    }
}

impl TransformTrait for SRTTransform {
    fn generate_transform(&self, time_t: f32) -> Transform {
        self.translate * self.rotate * self.scale
    }
}


pub fn three_sphere(time_0: f32, time_1: f32) -> Scene {
    Scene {
        // primatives: vec![
        //     Primative {
        //         shape: Arc::new(Sphere {radius: 1.0}),
        //         transform: Arc::new(SRTTransform::init(
        //             Vector3f {x: 0.0, y: 0.0, z: 0.0},
        //             3.0,
        //             3.0,
        //             3.0,
        //             0.0,
        //             Vector3f {x: 0.0, y: 1.0, z: 0.0},
        //         )),
        //     },
        //     Primative {
        //         shape: Arc::new(Sphere {radius: 1.0}),
        //         transform: Arc::new(SRTTransform::init(
        //             Vector3f {x: -6.0, y: 0.0, z: 0.0},
        //             2.0,
        //             2.0,
        //             2.0,
        //             0.0,
        //             Vector3f {x: 0.0, y: 1.0, z: 0.0},
        //         )),
        //     },
        //     Primative {
        //         shape: Arc::new(Sphere {radius: 1.0}),
        //         transform: Arc::new(SRTTransform::init(
        //             Vector3f {x: 5.0, y: 0.0, z: 0.0},
        //             1.0,
        //             1.0,
        //             1.0,
        //             0.0,
        //             Vector3f {x: 0.0, y: 1.0, z: 0.0},
        //         )),
        //     },
        // ],
        camera: Camera::create(
            Point3f {x: 1.5, y: 10.0, z: 10.0},
            Point3f {x: 0.0, y: 0.0, z: 0.0},
            Vector3f {x: 0.0, y:1.0, z:0.0},
            90.0,
            2.0,
            0.1,
            10.0,
            time_0,
            time_1,
        ),
    }
}

// pub fn rotating_sphere_animation(time_0: f32, time_1: f32) -> Scene {}
