use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
};
use std::cmp::PartialEq;

use std::clone::Clone;
use serde::{Deserialize, Serialize};

use crate::vector::Vec3;
use crate::core::Vector3f;
use crate::core::Normal3f;


#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Point2T<T> {
    pub x : T,
    pub y : T,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Point3T<T> {
    pub x : T,
    pub y : T,
    pub z : T,
}

pub type Point2i = Point2T<i32>;
pub type Point2f = Point2T<f32>;
pub type Point3i = Point3T<i32>;
pub type Point3f = Point3T<f32>;

impl Point2T<f32> {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y ).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl Point3T<f32> {
    pub fn distance(a: &Point3T<f32>, b: &Point3T<f32>) -> f32 {
        (a - b).length()
    }

    pub fn squared_distance(a: &Point3T<f32>, b: &Point3T<f32>) -> f32 {
        (a - b).squared_length()
    }
}

impl<T> From<Point3T<T>> for Point2T<T> {
    fn from(from: Point3T<T>) -> Self {
        Point2T {x: from.x, y: from.y}
    }
}

impl From<Point3f> for Point3i {
    fn from(from: Point3f) -> Self {
        Point3T {x: from.x as i32, y: from.y as i32, z: from.z as i32}
    }
}

impl From<Point3i> for Point3f {
    fn from(from: Point3i) -> Self {
        Point3T {x: from.x as f32, y: from.y as f32, z: from.z as f32}
    }
}

impl From<Point2f> for Point2i {
    fn from(from: Point2f) -> Self {
        Point2T {x: from.x as i32, y: from.y as i32}
    }
}

impl From<Point2i> for Point2f {
    fn from(from: Point2i) -> Self {
        Point2T {x: from.x as f32, y: from.y as f32}
    }
}

impl From<Point3f> for Vec3 {
    fn from(from: Point3f) -> Self {
        Vec3 { e: [ from.x, from.y, from.z ]}
    }
}

impl From<&Point3f> for Vec3 {
    fn from(from: &Point3f) -> Self {
        Vec3 { e: [ from.x, from.y, from.z ]}
    }
}

impl From<Vec3> for Point3f {
    fn from(from: Vec3) -> Self {
        Point3f {
            x: from.x(),
            y: from.y(),
            z: from.z(),
        }
    }
}

impl From<&Vec3> for Point3f {
    fn from(from: &Vec3) -> Self {
        Point3f {
            x: from.x(),
            y: from.y(),
            z: from.z(),
        }
    }
}

impl Add<Vec3> for Point3f {
    type Output = Point3f;

    fn add(self, _rhs: Vec3) -> Point3f {
        Point3f {
            x: self.x + _rhs.e[0],
            y: self.y + _rhs.e[1],
            z: self.z + _rhs.e[2],
        }
    }
}

impl Add<&Vec3> for &Point3f {
    type Output = Point3f;

    fn add(self, _rhs: &Vec3) -> Point3f {
        Point3f {
            x: self.x + _rhs.e[0],
            y: self.y + _rhs.e[1],
            z: self.z + _rhs.e[2],
        }
    }
}

impl AddAssign<&Vec3> for Point3f {
    fn add_assign(&mut self, _rhs: &Vec3) {
        self.x += _rhs.e[0];
        self.y += _rhs.e[1];
        self.z += _rhs.e[2];
    }
}

impl Sub<Vec3> for Point3f {
    type Output = Point3f;

    fn sub(self, _rhs: Vec3) -> Point3f {
        Point3f {
            x: self.x - _rhs.e[0],
            y: self.y - _rhs.e[1],
            z: self.z - _rhs.e[2],
        }
    }
}

impl Sub<&Vec3> for Point3f {
    type Output = Point3f;

    fn sub(self, _rhs: &Vec3) -> Point3f {
        Point3f {
            x: self.x - _rhs.e[0],
            y: self.y - _rhs.e[1],
            z: self.z - _rhs.e[2],
        }
    }
}

impl Sub<&Vec3> for &Point3f {
    type Output = Point3f;

    fn sub(self, _rhs: &Vec3) -> Point3f {
        Point3f {
            x: self.x - _rhs.e[0],
            y: self.y - _rhs.e[1],
            z: self.z - _rhs.e[2],
        }
    }
}

impl SubAssign<&Vec3> for Point3f {
    fn sub_assign(&mut self, _rhs: &Vec3) {
        self.x -= _rhs.e[0];
        self.y -= _rhs.e[1];
        self.z -= _rhs.e[2];
    }
}


// New definitions to replace Vec3

impl Add<Vector3f> for Point3f {
    type Output = Point3f;

    fn add(self, _rhs: Vector3f) -> Point3f {
        Point3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Add<&Vector3f> for Point3f {
    type Output = Point3f;

    fn add(self, _rhs: &Vector3f) -> Point3f {
        Point3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Add<Vector3f> for &Point3f {
    type Output = Point3f;

    fn add(self, _rhs: Vector3f) -> Point3f {
        Point3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Add<&Vector3f> for &Point3f {
    type Output = Point3f;

    fn add(self, _rhs: &Vector3f) -> Point3f {
        Point3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Add<Normal3f> for Point3f {
    type Output = Point3f;

    fn add(self, _rhs: Normal3f) -> Point3f {
        Point3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Add<&Normal3f> for Point3f {
    type Output = Point3f;

    fn add(self, _rhs: &Normal3f) -> Point3f {
        Point3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Add<Normal3f> for &Point3f {
    type Output = Point3f;

    fn add(self, _rhs: Normal3f) -> Point3f {
        Point3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Add<&Normal3f> for &Point3f {
    type Output = Point3f;

    fn add(self, _rhs: &Normal3f) -> Point3f {
        Point3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl AddAssign<&Normal3f> for Point3f {
    fn add_assign(&mut self, _rhs: &Normal3f) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

impl AddAssign<&Vector3f> for Point3f {
    fn add_assign(&mut self, _rhs: &Vector3f) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

impl Sub for Point3f {
    type Output = Vector3f;

    fn sub(self, _rhs: Point3f) -> Vector3f {
        Vector3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Sub<&Point3f> for Point3f {
    type Output = Vector3f;

    fn sub(self, _rhs: &Point3f) -> Vector3f {
        Vector3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Sub for &Point3f {
    type Output = Vector3f;

    fn sub(self, _rhs: &Point3f) -> Vector3f {
        Vector3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Sub<Point3f> for &Point3f {
    type Output = Vector3f;

    fn sub(self, _rhs: Point3f) -> Vector3f {
        Vector3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Sub<Vector3f> for Point3f {
    type Output = Point3f;

    fn sub(self, _rhs: Vector3f) -> Point3f {
        Point3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Sub<&Vector3f> for Point3f {
    type Output = Point3f;

    fn sub(self, _rhs: &Vector3f) -> Point3f {
        Point3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Sub<&Vector3f> for &Point3f {
    type Output = Point3f;

    fn sub(self, _rhs: &Vector3f) -> Point3f {
        Point3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl SubAssign<&Vector3f> for Point3f {
    fn sub_assign(&mut self, _rhs: &Vector3f) {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
        self.z -= _rhs.z;
    }
}

impl PartialEq for Point3f {
    fn eq(&self, rhs: &Point3f) -> bool {
        self.x == rhs.x &&
        self.y == rhs.y &&
        self.z == rhs.z
    }
}

// Vector3<T> operator-(const Point3<T> &p) const {
//     return Vector3<T>(x - p.x, y - p.y, z - p.z);
// }
// Subtracting a vector from a point gives a new point.

// <<Point3 Public Methods>>+= 
// Point3<T> operator-(const Vector3<T> &v) const {
//     return Point3<T>(x - v.x, y - v.y, z - v.z);
// }
// Point3<T> &operator-=(const Vector3<T> &v) {
//     x -= v.x; y -= v.y; z -= v.z;
//     return *this;
// }