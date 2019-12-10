use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
};

use std::clone::Clone;
use serde::{Deserialize, Serialize};

use crate::vector::Vec3;


#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Vector2T<T> {
    pub x : T,
    pub y : T,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Vector3T<T> {
    pub x : T,
    pub y : T,
    pub z : T,
}

pub type Vector2i = Vector2T<i32>;
pub type Vector2f = Vector2T<f32>;
pub type Vector3i = Vector3T<i32>;
pub type Vector3f = Vector3T<f32>;

impl Vector2f {
    pub fn length<T>(&self) -> f32 {
        (self.x * self.x + self.y * self.y ).sqrt()
    }

    pub fn squared_length<T>(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl Vector3f {
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

pub fn dot(l: &Vector3f, r: &Vector3f) -> f32
    // where T: Mul + Copy + Add,
    //       <T as Mul>::Output: Add<Output=T>
{
    l.x * r.x + l.y * r.y + l.z * r.z
}

// pub fn cross<T: Mul + Sub>(l: &Vector3T<T>, r: &Vector3T<T>) -> Vector3T<T>
//     where T: Mul + Copy + Sub,
//           <T as Mul>::Output: Sub<Output=T>
pub fn cross(l: &Vector3f, r: &Vector3f) -> Vector3f
{
    Vector3f {
        x: l.y * r.z - l.z * r.y,
        y: l.z * r.x - l.x * r.z,
        z: l.x * r.y - l.y * r.x,
    }
}

impl Add for Vector3f {
    type Output = Vector3f;

    fn add(self, _rhs: Vector3f) -> Vector3f {
        Vector3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}


impl Add<&Vector3f> for &Vector3f {
    type Output = Vector3f;

    fn add(self, _rhs: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}


impl AddAssign for Vector3f {
    fn add_assign(&mut self, _rhs: Vector3f) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

impl Sub for Vector3f {
    type Output = Vector3f;

    fn sub(self, _rhs: Vector3f) -> Vector3f {
        Vector3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}


impl Sub<&Vector3f> for &Vector3f {
    type Output = Vector3f;

    fn sub(self, _rhs: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}


impl SubAssign for Vector3f {
    fn sub_assign(&mut self, _rhs: Vector3f) {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
        self.z -= _rhs.z;
    }
}


impl Mul<f32> for Vector3f {
    type Output = Vector3f;

    fn mul(self, _rhs: f32) -> Vector3f {
        Vector3f {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}


impl MulAssign<f32> for Vector3f {
    fn mul_assign(&mut self, _rhs: f32) {
        self.x *= _rhs;
        self.y *= _rhs;
        self.z *= _rhs;
    }
}


impl Div<f32> for Vector3f {
    type Output = Vector3f;

    fn div(self, _rhs: f32) -> Vector3f {
        Vector3f {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}


impl DivAssign<f32> for Vector3f {
    fn div_assign(&mut self, _rhs: f32) {
        self.x /= _rhs;
        self.y /= _rhs;
        self.z /= _rhs;
    }
}





// impl Div for Vector3f {
//     type Output = Vector3f;

//     fn div(self, _rhs: Vector3f) -> Vector3f {
//         Vec3 { e: [
//             self.e[0] / _rhs.e[0],
//             self.e[1] / _rhs.e[1],
//             self.e[2] / _rhs.e[2],
//         ]}
//     }
// }

// impl DivAssign for Vec3 {
//     fn div_assign(&mut self, _rhs: Vec3) {
//         *self = Vec3 { e: [
//             self.e[0] / _rhs.e[0],
//             self.e[1] / _rhs.e[1],
//             self.e[2] / _rhs.e[2],
//         ]};
//     }
// }