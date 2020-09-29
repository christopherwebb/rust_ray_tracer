use crate::core::matrix::Matrix4x4f as Matrix;
use crate::core::vector::Vector3f;
use std::ops::Mul;

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

    let (sin_theta, cos_theta) = (-theta).sin_cos();
    let inverse_matrix = Matrix {
        m: [
            [1.0,       0.0,        0.0, 0.0],
            [0.0, cos_theta, -sin_theta, 0.0],
            [0.0, sin_theta,  cos_theta, 0.0],
            [0.0,       0.0,        0.0, 1.0],
        ]
    };

    Transform {
        m: rotate_matrix,
        m_inv: inverse_matrix,
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

    let (sin_theta, cos_theta) = (-theta).sin_cos();
    let inverse_matrix = Matrix {
        m: [
            [ cos_theta, 0.0, sin_theta, 0.0],
            [       0.0, 1.0,       0.0, 0.0],
            [-sin_theta, 0.0, cos_theta, 0.0],
            [       0.0, 0.0,       0.0, 1.0],
        ]
    };

    Transform {
        m: rotate_matrix,
        m_inv: inverse_matrix,
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

    let (sin_theta, cos_theta) = (-theta).sin_cos();
    let inverse_matrix = Matrix {
        m: [
            [ cos_theta, -sin_theta, 0.0, 0.0],
            [ sin_theta,  cos_theta, 0.0, 0.0],
            [       0.0,        0.0, 1.0, 0.0],
            [       0.0,        0.0, 0.0, 1.0],
        ]
    };

    Transform {
        m: rotate_matrix,
        m_inv: inverse_matrix,
    }
}

fn general_rotate_matrix(theta: f32, unit_a: Vector3f) -> Matrix {
    let (sin_theta, cos_theta) = theta.sin_cos();

    Matrix {
        m: [
            [
                unit_a.x * unit_a.x + (1.0 - unit_a.x * unit_a.x) * cos_theta,
                unit_a.x * unit_a.y * (1.0 - cos_theta) - unit_a.z * sin_theta,
                unit_a.x * unit_a.z * (1.0 - cos_theta) + unit_a.y * sin_theta,
                0.0,
            ],
            [
                unit_a.y * unit_a.x * (1.0 - cos_theta) + unit_a.z * sin_theta,
                unit_a.y * unit_a.y + (1.0 - unit_a.y * unit_a.y) * cos_theta,
                unit_a.y * unit_a.z * (1.0 - cos_theta) - unit_a.x * sin_theta,
                0.0,
            ],
            [
                unit_a.z * unit_a.x * (1.0 - cos_theta) - unit_a.y * sin_theta,
                unit_a.z * unit_a.y * (1.0 - cos_theta) + unit_a.x * sin_theta,
                unit_a.z * unit_a.z + (1.0 - unit_a.z * unit_a.z) * cos_theta,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}

pub fn gen_rotate(theta: f32, axis: Vector3f) -> Transform {
    // normalise_axis
    let a = axis.unit_vector();

    // let (sin_theta, cos_theta) = theta.sin_cos();

    let rotate_matrix = general_rotate_matrix(theta, a);
    let inverse_matrix = general_rotate_matrix(-theta, a);

    Transform {
        m: rotate_matrix,
        m_inv: inverse_matrix,
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
