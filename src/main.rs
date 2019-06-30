use std::io::{self, Write};
use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign
};
use std::clone::Clone;
// use std::num::Float;

#[derive(Clone)]
struct Vec3 {
    e : [f32; 3],
}

impl Vec3 {
    fn x(&self) -> f32 { return self.e[0]; }
    fn y(&self) -> f32 { return self.e[1]; }
    fn z(&self) -> f32 { return self.e[2]; }
    fn r(&self) -> f32 { return self.e[0]; }
    fn g(&self) -> f32 { return self.e[1]; }
    fn b(&self) -> f32 { return self.e[2]; }

    fn length(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 { e: [
            self.e[0] + _rhs.e[0],
            self.e[1] + _rhs.e[1],
            self.e[2] + _rhs.e[2],
        ]}
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        *self = Vec3 { e: [
            self.e[0] + _rhs.e[0],
            self.e[1] + _rhs.e[1],
            self.e[2] + _rhs.e[2],
        ]};
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 { e: [
            self.e[0] - _rhs.e[0],
            self.e[1] - _rhs.e[1],
            self.e[2] - _rhs.e[2],
        ]}
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: &Vec3) -> Vec3 {
        Vec3 { e: [
            self.e[0] - _rhs.e[0],
            self.e[1] - _rhs.e[1],
            self.e[2] - _rhs.e[2],
        ]}
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, _rhs: Vec3) {
        *self = Vec3 { e: [
            self.e[0] - _rhs.e[0],
            self.e[1] - _rhs.e[1],
            self.e[2] - _rhs.e[2],
        ]};
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 { e: [
            self.e[0] * _rhs.e[0],
            self.e[1] * _rhs.e[1],
            self.e[2] * _rhs.e[2],
        ]}
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3 { e: [
            self.e[0] * _rhs,
            self.e[1] * _rhs,
            self.e[2] * _rhs,
        ]}
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 { e: [
            self * _rhs.e[0],
            self * _rhs.e[1],
            self * _rhs.e[2],
        ]}
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, _rhs: Vec3) {
        *self = Vec3 { e: [
            self.e[0] * _rhs.e[0],
            self.e[1] * _rhs.e[1],
            self.e[2] * _rhs.e[2],
        ]};
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, _rhs: f32) {
        *self = Vec3 { e: [
            self.e[0] * _rhs,
            self.e[1] * _rhs,
            self.e[2] * _rhs,
        ]};
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 { e: [
            self.e[0] / _rhs.e[0],
            self.e[1] / _rhs.e[1],
            self.e[2] / _rhs.e[2],
        ]}
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3 { e: [
            self.e[0] / _rhs,
            self.e[1] / _rhs,
            self.e[2] / _rhs,
        ]}
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, _rhs: Vec3) {
        *self = Vec3 { e: [
            self.e[0] / _rhs.e[0],
            self.e[1] / _rhs.e[1],
            self.e[2] / _rhs.e[2],
        ]};
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, _rhs: f32) {
        *self = Vec3 { e: [
            self.e[0] / _rhs,
            self.e[1] / _rhs,
            self.e[2] / _rhs,
        ]};
    }
}

fn dot(l: &Vec3, r: &Vec3) -> f32 {
    l.e[0] * r.e[0] + l.e[1] * r.e[1] + l.e[2] * r.e[2]
}

fn cross(l: Vec3, r: Vec3) -> Vec3 {
    Vec3 { e: [
        l.e[1] * r.e[2] - l.e[2] * r.e[1],
        l.e[0] * r.e[2] - l.e[2] * r.e[0],
        l.e[0] * r.e[1] - l.e[1] * r.e[0],
    ]}
}

fn unit_vector(vec : Vec3) -> Vec3 {
    let length = vec.length();
    vec.clone() / length
}

struct Ray {
    a : Vec3,
    b : Vec3,
}

impl Ray {
    fn origin(&self) -> Vec3 { self.a.clone() }
    fn direction(&self) -> Vec3 { self.b.clone() }
    fn point_at_parameter(self, point : f32) -> Vec3 { self.a + point * self.b }
}

fn hit_sphere(centre: &Vec3, radius: f32, ray: &Ray) -> bool {
    let oc : Vec3 = ray.origin() - centre;

    let a : f32 = dot(&ray.direction(), &ray.direction());
    let b : f32 = 2.0 * dot(&oc, &ray.direction());
    let c : f32 = dot(&oc, &oc) - radius * radius;

    let discriminant : f32 = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn colour(r : &Ray) -> Vec3 {
    if hit_sphere(&Vec3 { e: [0.0, 0.0, -1.0]}, 0.5, r) {
        return Vec3 { e: [1.0, 0.0, 0.0]}
    }
    let dir : Vec3 = r.direction();
    let unit_dir : Vec3 = unit_vector(dir);
    let t : f32 = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vec3 { e: [1.0, 1.0, 1.0]} + t * Vec3 { e: [0.5, 0.7, 1.0]}
}

fn main() {
    let nX = 200;
    let nY = 100;

    let lower_left_corner : Vec3 = Vec3 { e: [-2.0, -1.0, -1.0]};
    let horizontal : Vec3 = Vec3 { e: [4.0, 0.0, 0.0]};
    let vertical : Vec3 = Vec3 { e: [0.0, 2.0, 0.0]};
    let origin : Vec3 = Vec3 { e: [0.0, 0.0, 0.0]};

    println!("P3\n{} {}\n255", nX, nY);
    for y_coord in (0..nY).rev() {
        for x_coord in 0..nX {
            let u: f32 = x_coord as f32 / nX as f32;
            let v: f32 = y_coord as f32 / nY as f32;

            let ray : Ray = Ray {
                a: origin.clone(),
                b: lower_left_corner.clone() + u * horizontal.clone() + v * vertical.clone()
            };

            let col: Vec3 = colour(&ray);

            let ir = (255.99 * col.r()) as u64;
            let ig = (255.99 * col.g()) as u64;
            let ib = (255.99 * col.b()) as u64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
