use std::ops::{
    // Add,
    // AddAssign,
    // Neg,
    // Sub,
    // SubAssign,
    Mul,
    // MulAssign,
    // Div,
    // DivAssign,
};
use std::clone::Clone;


#[derive(Clone, Copy, Debug)]
pub struct Matrix4x4<T> {
    // pub m : T[4][4],
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
