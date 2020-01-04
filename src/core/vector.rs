use std::ops::{
    Add,
    AddAssign,
    Neg,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
};

use std::clone::Clone;

use rand::thread_rng;
use rand::Rng;

use serde::{Deserialize, Serialize};

use crate::vector::Vec3;
use crate::core::Point3f;
use crate::core::Normal3f;
use crate::core::dot_vv;


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

    pub fn rnd_in_unit_sphere() -> Self {
        let mut rng = thread_rng();
        let sub = Self { x: 1.0, y: 1.0, z: 1.0 };

        loop {
            let x = 2.0 * Self {
                x: rng.gen::<f64>() as f32,
                y: rng.gen::<f64>() as f32,
                z: rng.gen::<f64>() as f32,
            } - sub;
            if x.squared_length() < 1.0 {
                break x;
            }
        }
    }

    pub fn rnd_in_unit_disc() -> Self {
        let mut rng = thread_rng();
        let sub = Self { x: 1.0, y: 1.0, z: 1.0 };

        loop {
            let p = 2.0 * Self {
                x: rng.gen::<f64>() as f32,
                y: rng.gen::<f64>() as f32,
                z: 0.0,
            } - sub;

            if dot_vv(&p, &p) < 1.0 {
                break p;
            }
        }
    }
}

pub fn cross<T>(l: &Vector3T<T>, r: &Vector3T<T>) -> Vector3T<T>
    where T: Mul + Copy + Sub,
          T: Mul<Output=T>,
          T: Sub<Output=T>,
{
    Vector3T {
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

impl Neg for Vector3f {
    type Output = Vector3f;

    fn neg(self) -> Vector3f {
        Vector3f {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
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

impl Sub<&Vector3f> for Vector3f {
    type Output = Vector3f;

    fn sub(self, _rhs: &Vector3f) -> Vector3f {
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

impl Sub<Vector3f> for &Vector3f {
    type Output = Vector3f;

    fn sub(self, _rhs: Vector3f) -> Vector3f {
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

impl Sub<Normal3f> for Vector3f {
    type Output = Vector3f;

    fn sub(self, _rhs: Normal3f) -> Vector3f {
        Vector3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Sub<&Normal3f> for Vector3f {
    type Output = Vector3f;

    fn sub(self, _rhs: &Normal3f) -> Vector3f {
        Vector3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}


impl Sub<&Normal3f> for &Vector3f {
    type Output = Vector3f;

    fn sub(self, _rhs: &Normal3f) -> Vector3f {
        Vector3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Sub<Normal3f> for &Vector3f {
    type Output = Vector3f;

    fn sub(self, _rhs: Normal3f) -> Vector3f {
        Vector3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl SubAssign<Normal3f> for Vector3f {
    fn sub_assign(&mut self, _rhs: Normal3f) {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
        self.z -= _rhs.z;
    }
}

impl SubAssign<&Normal3f> for Vector3f {
    fn sub_assign(&mut self, _rhs: &Normal3f) {
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


impl Mul<Vector3f> for f32 {
    type Output = Vector3f;

    fn mul(self, _rhs: Vector3f) -> Vector3f {
        Vector3f {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}


impl Mul<&Vector3f> for f32 {
    type Output = Vector3f;

    fn mul(self, _rhs: &Vector3f) -> Vector3f {
        Vector3f {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
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


impl From<Point3f> for Vector3f {
    fn from(from: Point3f) -> Self {
        Vector3f {x: from.x, y: from.y, z: from.z}
    }
}


impl From<&Point3f> for Vector3f {
    fn from(from: &Point3f) -> Self {
        Vector3f {x: from.x, y: from.y, z: from.z}
    }
}

impl From<Normal3f> for Vector3f {
    fn from(from: Normal3f) -> Self {
        Vector3f {x: from.x, y: from.y, z: from.z}
    }
}

impl From<&Normal3f> for Vector3f {
    fn from(from: &Normal3f) -> Self {
        Vector3f {x: from.x, y: from.y, z: from.z}
    }
}

impl From<Vector3f> for Vec3 {
    fn from(from: Vector3f) -> Self {
        Vec3 { e: [ from.x, from.y, from.z ]}
    }
}

impl From<&Vector3f> for Vec3 {
    fn from(from: &Vector3f) -> Self {
        Vec3 { e: [ from.x, from.y, from.z ]}
    }
}

impl From<Vec3> for Vector3f {
    fn from(from: Vec3) -> Self {
        Vector3f {
            x: from.x(),
            y: from.y(),
            z: from.z(),
        }
    }
}

impl From<&Vec3> for Vector3f {
    fn from(from: &Vec3) -> Self {
        Vector3f {
            x: from.x(),
            y: from.y(),
            z: from.z(),
        }
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