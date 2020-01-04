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
