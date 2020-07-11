use crate::core::matrix::{
    Matrix4x4f as Matrix,
};
use crate::core::vector::Vector3f;
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

pub struct Transform {
    pub m: Matrix,
    pub m_inv: Matrix,
}

pub fn gen_translate(delta: Vector3f) -> Transform {
    Transform {
        m: Matrix {
            m: [
                [1.0, 0.0, 0.0, delta.x],
                [0.0, 1.0, 0.0, delta.y],
                [0.0, 0.0, 1.0, delta.z],
                [0.0, 0.0, 0.0,     1.0],
            ]
        },
        m_inv: Matrix {
            m: [
                [1.0, 0.0, 0.0, -delta.x],
                [0.0, 1.0, 0.0, -delta.y],
                [0.0, 0.0, 1.0, -delta.z],
                [0.0, 0.0, 0.0,      1.0],
            ]
        },
    }
}

pub fn gen_scale(x: f32, y: f32, z: f32) -> Transform {
    Transform {
        m: Matrix {
            m: [
                [  x, 0.0, 0.0, 0.0],
                [0.0,   y, 0.0, 0.0],
                [0.0, 0.0,   z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        },
        m_inv: Matrix {
            m: [
                [1.0 / x,     0.0,     0.0, 0.0],
                [    0.0, 1.0 / y,     0.0, 0.0],
                [    0.0,     0.0, 1.0 / z, 0.0],
                [    0.0,     0.0,     0.0, 1.0],
            ]
        },
    }
}

pub fn gen_rotate_x(theta: f32) -> Transform {
    let (sin_theta, cos_theta) = theta.sin_cos();
    let rotate_matrix = Matrix {
        m: [
            [1.0,       0.0,        0.0, 0.0],
            [0.0, cos_theta, -sin_theta, 0.0],
            [0.0, sin_theta,  cos_theta, 0.0],
            [0.0,       0.0,        0.0, 1.0],
        ]
    };

    Transform {
        m: rotate_matrix,
        m_inv: rotate_matrix.transpose(),
    }
}

pub fn gen_rotate_y(theta: f32) -> Transform {
    let (sin_theta, cos_theta) = theta.sin_cos();
    let rotate_matrix = Matrix {
        m: [
            [ cos_theta, 0.0, sin_theta, 0.0],
            [       0.0, 1.0,       0.0, 0.0],
            [-sin_theta, 0.0, cos_theta, 0.0],
            [       0.0, 0.0,       0.0, 1.0],
        ]
    };

    Transform {
        m: rotate_matrix,
        m_inv: rotate_matrix.transpose(),
    }
}

pub fn gen_rotate_z(theta: f32) -> Transform {
    let (sin_theta, cos_theta) = theta.sin_cos();
    let rotate_matrix = Matrix {
        m: [
            [ cos_theta, -sin_theta, 0.0, 0.0],
            [ sin_theta,  cos_theta, 0.0, 0.0],
            [       0.0,        0.0, 1.0, 0.0],
            [       0.0,        0.0, 0.0, 1.0],
        ]
    };

    Transform {
        m: rotate_matrix,
        m_inv: rotate_matrix.transpose(),
    }
}

pub fn gen_rotate(theta: f32, axis: Vector3f) -> Transform {
    // normalise_axis
    let a = axis.unit_vector();
    let (sin_theta, cos_theta) = theta.sin_cos();

    let rotate_matrix = Matrix {
        m: [
            [
                a.x * a.x + (1.0 - a.x * a.x) * cos_theta,
                a.x * a.y * (1.0 - cos_theta) - a.z * sin_theta,
                a.x * a.z * (1.0 - cos_theta) + a.y * sin_theta,
                0.0,
            ],
            [
                a.y * a.x * (1.0 - cos_theta) + a.z * sin_theta,
                a.y * a.y + (1.0 - a.y * a.y) * cos_theta,
                a.y * a.z * (1.0 - cos_theta) - a.x * sin_theta,
                0.0,
            ],
            [
                a.z * a.x * (1.0 - cos_theta) - a.y * sin_theta,
                a.z * a.y * (1.0 - cos_theta) + a.x * sin_theta,
                a.z * a.z + (1.0 - a.z * a.z) * cos_theta,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ]
    };

    Transform {
        m: rotate_matrix,
        m_inv: rotate_matrix.transpose(),
    }
}

impl Mul for Transform {
    type Output = Transform;
    fn mul(self, _rhs: Transform) -> Transform {
        Transform {
            m: self.m * _rhs.m,
            m_inv: _rhs.m_inv * self.m_inv,
        }
    }
}
