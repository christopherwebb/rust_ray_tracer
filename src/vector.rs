use std::ops::{
    Add,
    AddAssign,
    Neg,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign
};
use std::clone::Clone;
use rand::thread_rng;
use rand::Rng;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Vec3 {
    pub e : [f32; 3],
}

impl Vec3 {
    pub fn x(&self) -> f32 { return self.e[0]; }
    pub fn y(&self) -> f32 { return self.e[1]; }
    pub fn z(&self) -> f32 { return self.e[2]; }
    pub fn r(&self) -> f32 { return self.e[0]; }
    pub fn g(&self) -> f32 { return self.e[1]; }
    pub fn b(&self) -> f32 { return self.e[2]; }

    pub fn length(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
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

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: &Vec3) -> Vec3 {
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

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { e: [
            -self.e[0],
            -self.e[1],
            -self.e[2],
        ]}
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

impl Sub<&Vec3> for &Vec3 {
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

impl Mul<f32> for &Vec3 {
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

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: &Vec3) -> Vec3 {
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

pub fn dot(l: &Vec3, r: &Vec3) -> f32 {
    l.e[0] * r.e[0] + l.e[1] * r.e[1] + l.e[2] * r.e[2]
}

pub fn cross(l: &Vec3, r: &Vec3) -> Vec3 {
    Vec3 { e: [
        l.e[1] * r.e[2] - l.e[2] * r.e[1],
        l.e[2] * r.e[0] - l.e[0] * r.e[2],
        l.e[0] * r.e[1] - l.e[1] * r.e[0],
    ]}
}

pub fn unit_vector(vec : &Vec3) -> Vec3 {
    let length = vec.length();
    vec.clone() / length
}

pub fn rnd_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();

    // let mut x : Vec3 = Vec3 {e: [4.0, 4.0, 4.0]};

    // while {
    //     x = 2.0 * Vec3{
    //         e: [
    //             rng.gen::<f64>() as f32,
    //             rng.gen::<f64>() as f32,
    //             rng.gen::<f64>() as f32,
    //         ]
    //     } - Vec3{ e: [1.0, 1.0, 1.0]};
    //     x.squared_length() >= 1.0
    // } {}

    // return x;

    loop {
        let x = 2.0 * Vec3{
            e: [
                rng.gen::<f64>() as f32,
                rng.gen::<f64>() as f32,
                rng.gen::<f64>() as f32,
            ]
        } - Vec3{ e: [1.0, 1.0, 1.0]};
        if x.squared_length() < 1.0 {
            break x;
        }
    }     
}

pub fn rnd_in_unit_disc() -> Vec3 {
    let mut rng = thread_rng();
    let sub = Vec3 { e: [1.0, 1.0, 0.0]};
    loop {

        let p = 2.0 * Vec3 { e: [
            rng.gen::<f64>() as f32,
            rng.gen::<f64>() as f32,
            0.0,
        ]} - &sub;

        if dot(&p, &p) < 1.0 {
            break p;
        }
    }
}

// pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
//     v - &(2.0 * dot(v, n) * n)
// }

pub fn refract(v : &Vec3, n : &Vec3, ni_over_nt : f32) -> (bool, Option<Vec3>) {
    let uv : Vec3 = unit_vector(v);
    let dt : f32 = dot(&uv, n);
    let discriminant : f32 = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        (true, Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt()))
    } else {
        (false, None)
    }
}
