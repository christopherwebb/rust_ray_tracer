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
use serde::{Deserialize, Serialize};

use crate::vector::Vec3;
use crate::core::Vector3f;


#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Normal2T<T> {
    pub x : T,
    pub y : T,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Normal3T<T> {
    pub x : T,
    pub y : T,
    pub z : T,
}

pub type Normal2i = Normal2T<i32>;
pub type Normal2f = Normal2T<f32>;
pub type Normal3i = Normal3T<i32>;
pub type Normal3f = Normal3T<f32>;

impl Normal3f {
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

pub fn dot<T>(l: &Normal3T<T>, r: &Normal3T<T>) -> T
    where T: Mul + Copy + Add,
          T: Mul<Output=T>,
          T: Add<Output=T>,

{
    l.x * r.x + l.y * r.y + l.z * r.z
}

impl From<Vector3f> for Normal3f {
    fn from(from: Vector3f) -> Self {
        Normal3f {x: from.x, y: from.y, z: from.z}
    }
}

impl From<&Vector3f> for Normal3f {
    fn from(from: &Vector3f) -> Self {
        Normal3f {x: from.x, y: from.y, z: from.z}
    }
}

impl Add for Normal3f {
    type Output = Normal3f;

    fn add(self, _rhs: Normal3f) -> Normal3f {
        Normal3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}


impl Add<&Normal3f> for &Normal3f {
    type Output = Normal3f;

    fn add(self, _rhs: &Normal3f) -> Normal3f {
        Normal3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}


impl AddAssign for Normal3f {
    fn add_assign(&mut self, _rhs: Normal3f) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

impl Neg for Normal3f {
    type Output = Normal3f;

    fn neg(self) -> Normal3f {
        Normal3f {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Normal3f {
    type Output = Normal3f;

    fn sub(self, _rhs: Normal3f) -> Normal3f {
        Normal3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}


impl Sub<&Normal3f> for &Normal3f {
    type Output = Normal3f;

    fn sub(self, _rhs: &Normal3f) -> Normal3f {
        Normal3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}


impl SubAssign for Normal3f {
    fn sub_assign(&mut self, _rhs: Normal3f) {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
        self.z -= _rhs.z;
    }
}


impl Mul<f32> for Normal3f {
    type Output = Normal3f;

    fn mul(self, _rhs: f32) -> Normal3f {
        Normal3f {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl Mul<f32> for &Normal3f {
    type Output = Normal3f;

    fn mul(self, _rhs: f32) -> Normal3f {
        Normal3f {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}


impl MulAssign<f32> for Normal3f {
    fn mul_assign(&mut self, _rhs: f32) {
        self.x *= _rhs;
        self.y *= _rhs;
        self.z *= _rhs;
    }
}


impl Mul<Normal3f> for f32 {
    type Output = Normal3f;

    fn mul(self, _rhs: Normal3f) -> Normal3f {
        Normal3f {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}


impl Mul<&Normal3f> for f32 {
    type Output = Normal3f;

    fn mul(self, _rhs: &Normal3f) -> Normal3f {
        Normal3f {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}


impl Div<f32> for Normal3f {
    type Output = Normal3f;

    fn div(self, _rhs: f32) -> Normal3f {
        Normal3f {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}


impl DivAssign<f32> for Normal3f {
    fn div_assign(&mut self, _rhs: f32) {
        self.x /= _rhs;
        self.y /= _rhs;
        self.z /= _rhs;
    }
}


// impl From<Point3f> for Normal3f {
//     fn from(from: Point3f) -> Self {
//         Normal3f {x: from.x, y: from.y, z: from.z}
//     }
// }


// impl From<&Point3f> for Normal3f {
//     fn from(from: &Point3f) -> Self {
//         Normal3f {x: from.x, y: from.y, z: from.z}
//     }
// }


impl From<Normal3f> for Vec3 {
    fn from(from: Normal3f) -> Self {
        Vec3 { e: [ from.x, from.y, from.z ]}
    }
}

impl From<&Normal3f> for Vec3 {
    fn from(from: &Normal3f) -> Self {
        Vec3 { e: [ from.x, from.y, from.z ]}
    }
}

impl From<Vec3> for Normal3f {
    fn from(from: Vec3) -> Self {
        Normal3f {
            x: from.x(),
            y: from.y(),
            z: from.z(),
        }
    }
}

impl From<&Vec3> for Normal3f {
    fn from(from: &Vec3) -> Self {
        Normal3f {
            x: from.x(),
            y: from.y(),
            z: from.z(),
        }
    }
}