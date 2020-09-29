use std::ops::Mul;
use std::cmp::PartialEq;
use std::clone::Clone;

use crate::core::{
    Normal3f,
    Point3f,
    Vector3f,
};


#[derive(Clone, Copy, Debug)]
pub struct Matrix4x4<T> {
    pub m : [[T; 4]; 4],
}

pub type Matrix4x4i = Matrix4x4<i32>;
pub type Matrix4x4f = Matrix4x4<f32>;

impl Matrix4x4f {
    pub fn transpose(self) -> Matrix4x4f {
        Matrix4x4f { m:
            [
                [self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0]],
                [self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1]],
                [self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2]],
                [self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]],
            ]
        }
    }
}

pub fn indentity() -> Matrix4x4f {
    Matrix4x4f { m:
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}

impl Mul for Matrix4x4f {
    type Output = Matrix4x4f;

    fn mul(self, _rhs: Matrix4x4f) -> Matrix4x4f {
        Matrix4x4f { m:
            [
                [
                    self.m[0][0] * _rhs.m[0][0] + self.m[0][1] * _rhs.m[1][0] + self.m[0][2] * _rhs.m[2][0] + self.m[0][3] * _rhs.m[3][0],
                    self.m[0][0] * _rhs.m[0][1] + self.m[0][1] * _rhs.m[1][1] + self.m[0][2] * _rhs.m[2][1] + self.m[0][3] * _rhs.m[3][1],
                    self.m[0][0] * _rhs.m[0][2] + self.m[0][1] * _rhs.m[1][2] + self.m[0][2] * _rhs.m[2][2] + self.m[0][3] * _rhs.m[3][2],
                    self.m[0][0] * _rhs.m[0][3] + self.m[0][1] * _rhs.m[1][3] + self.m[0][2] * _rhs.m[2][3] + self.m[0][3] * _rhs.m[3][3],
                ],
                [
                    self.m[1][0] * _rhs.m[0][0] + self.m[1][1] * _rhs.m[1][0] + self.m[1][2] * _rhs.m[2][0] + self.m[1][3] * _rhs.m[3][0],
                    self.m[1][0] * _rhs.m[0][1] + self.m[1][1] * _rhs.m[1][1] + self.m[1][2] * _rhs.m[2][1] + self.m[1][3] * _rhs.m[3][1],
                    self.m[1][0] * _rhs.m[0][2] + self.m[1][1] * _rhs.m[1][2] + self.m[1][2] * _rhs.m[2][2] + self.m[1][3] * _rhs.m[3][2],
                    self.m[1][0] * _rhs.m[0][3] + self.m[1][1] * _rhs.m[1][3] + self.m[1][2] * _rhs.m[2][3] + self.m[1][3] * _rhs.m[3][3],
                ],
                [
                    self.m[2][0] * _rhs.m[0][0] + self.m[2][1] * _rhs.m[1][0] + self.m[2][2] * _rhs.m[2][0] + self.m[2][3] * _rhs.m[3][0],
                    self.m[2][0] * _rhs.m[0][1] + self.m[2][1] * _rhs.m[1][1] + self.m[2][2] * _rhs.m[2][1] + self.m[2][3] * _rhs.m[3][1],
                    self.m[2][0] * _rhs.m[0][2] + self.m[2][1] * _rhs.m[1][2] + self.m[2][2] * _rhs.m[2][2] + self.m[2][3] * _rhs.m[3][2],
                    self.m[2][0] * _rhs.m[0][3] + self.m[2][1] * _rhs.m[1][3] + self.m[2][2] * _rhs.m[2][3] + self.m[2][3] * _rhs.m[3][3],
                ],
                [
                    self.m[3][0] * _rhs.m[0][0] + self.m[3][1] * _rhs.m[1][0] + self.m[3][2] * _rhs.m[2][0] + self.m[3][3] * _rhs.m[3][0],
                    self.m[3][0] * _rhs.m[0][1] + self.m[3][1] * _rhs.m[1][1] + self.m[3][2] * _rhs.m[2][1] + self.m[3][3] * _rhs.m[3][1],
                    self.m[3][0] * _rhs.m[0][2] + self.m[3][1] * _rhs.m[1][2] + self.m[3][2] * _rhs.m[2][2] + self.m[3][3] * _rhs.m[3][2],
                    self.m[3][0] * _rhs.m[0][3] + self.m[3][1] * _rhs.m[1][3] + self.m[3][2] * _rhs.m[2][3] + self.m[3][3] * _rhs.m[3][3],
                ],
            ]
        }
    }
}

impl Mul<Point3f> for Matrix4x4f {
    type Output = Point3f;

    fn mul(self, _rhs: Point3f) -> Point3f {
        Point3f {
            x: self.m[0][0] * _rhs.x + self.m[0][1] * _rhs.y + self.m[0][2] * _rhs.z + self.m[0][3],
            y: self.m[1][0] * _rhs.x + self.m[1][1] * _rhs.y + self.m[1][2] * _rhs.z + self.m[1][3],
            z: self.m[2][0] * _rhs.x + self.m[2][1] * _rhs.y + self.m[2][2] * _rhs.z + self.m[2][3],
        }

    }
}

impl Mul<Vector3f> for Matrix4x4f {
    type Output = Vector3f;

    fn mul(self, _rhs: Vector3f) -> Vector3f {
        Vector3f {
            x: self.m[0][0] * _rhs.x + self.m[0][1] * _rhs.y + self.m[0][2] * _rhs.z,
            y: self.m[1][0] * _rhs.x + self.m[1][1] * _rhs.y + self.m[1][2] * _rhs.z,
            z: self.m[2][0] * _rhs.x + self.m[2][1] * _rhs.y + self.m[2][2] * _rhs.z,
        }
    }
}

impl Mul<Normal3f> for Matrix4x4f {
    type Output = Normal3f;

    fn mul(self, _rhs: Normal3f) -> Normal3f {
        Normal3f {
            x: self.m[0][0] * _rhs.x + self.m[0][1] * _rhs.y + self.m[0][2] * _rhs.z,
            y: self.m[1][0] * _rhs.x + self.m[1][1] * _rhs.y + self.m[1][2] * _rhs.z,
            z: self.m[2][0] * _rhs.x + self.m[2][1] * _rhs.y + self.m[2][2] * _rhs.z,
        }

    }
}

impl PartialEq for Matrix4x4f {
    fn eq(&self, rhs: &Matrix4x4f) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Matrix4x4f;

    // Scenario: Transposing a matrix
    //   Given the following matrix A:
    //     | 0 | 9 | 3 | 0 |
    //     | 9 | 8 | 0 | 8 |
    //     | 1 | 8 | 5 | 3 |
    //     | 0 | 0 | 5 | 8 |
    //   Then transpose(A) is the following matrix:
    //     | 0 | 9 | 1 | 0 |
    //     | 9 | 8 | 8 | 0 |
    //     | 3 | 0 | 5 | 5 |
    //     | 0 | 8 | 3 | 8 |
    #[test]
    fn tranpose() {
        let m = Matrix4x4f { m:
            [
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0],
            ]
        };

        let transposed = m.transpose();
        assert_eq!(
            transposed,
            Matrix4x4f { m:
                [
                    [0.0, 9.0, 1.0, 0.0],
                    [9.0, 8.0, 8.0, 0.0],
                    [3.0, 0.0, 5.0, 5.0],
                    [0.0, 8.0, 3.0, 8.0],
                ]
            }
        );
    }
}