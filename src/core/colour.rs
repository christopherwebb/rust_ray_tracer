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

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Colour {
    pub r : f32,
    pub g : f32,
    pub b : f32,
}

impl Add for Colour {
    type Output = Colour;

    fn add(self, _rhs: Colour) -> Colour {
        Colour {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b,
        }
    }
}

impl Add<&Colour> for &Colour {
    type Output = Colour;

    fn add(self, _rhs: &Colour) -> Colour {
        Colour {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b,
        }
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, _rhs: Colour) {
        self.r += _rhs.r;
        self.g += _rhs.g;
        self.b += _rhs.b;
    }
}

impl Neg for Colour {
    type Output = Colour;

    fn neg(self) -> Colour {
        Colour {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

impl Sub for Colour {
    type Output = Colour;

    fn sub(self, _rhs: Colour) -> Colour {
        Colour {
            r: self.r - _rhs.r,
            g: self.g - _rhs.g,
            b: self.b - _rhs.b,
        }
    }
}

impl Sub<&Colour> for Colour {
    type Output = Colour;

    fn sub(self, _rhs: &Colour) -> Colour {
        Colour {
            r: self.r - _rhs.r,
            g: self.g - _rhs.g,
            b: self.b - _rhs.b,
        }
    }
}


impl Sub<&Colour> for &Colour {
    type Output = Colour;

    fn sub(self, _rhs: &Colour) -> Colour {
        Colour {
            r: self.r - _rhs.r,
            g: self.g - _rhs.g,
            b: self.b - _rhs.b,
        }
    }
}

impl Sub<Colour> for &Colour {
    type Output = Colour;

    fn sub(self, _rhs: Colour) -> Colour {
        Colour {
            r: self.r - _rhs.r,
            g: self.g - _rhs.g,
            b: self.b - _rhs.b,
        }
    }
}

impl SubAssign for Colour {
    fn sub_assign(&mut self, _rhs: Colour) {
        self.r -= _rhs.r;
        self.g -= _rhs.g;
        self.b -= _rhs.b;
    }
}

impl Mul<f32> for Colour {
    type Output = Colour;

    fn mul(self, _rhs: f32) -> Colour {
        Colour {
            r: self.r * _rhs,
            g: self.g * _rhs,
            b: self.b * _rhs,
        }
    }
}

impl MulAssign<f32> for Colour {
    fn mul_assign(&mut self, _rhs: f32) {
        self.r *= _rhs;
        self.g *= _rhs;
        self.b *= _rhs;
    }
}

impl Div<f32> for Colour {
    type Output = Colour;

    fn div(self, _rhs: f32) -> Colour {
        Colour {
            r: self.r / _rhs,
            g: self.g / _rhs,
            b: self.b / _rhs,
        }
    }
}

impl DivAssign<f32> for Colour {
    fn div_assign(&mut self, _rhs: f32) {
        self.r /= _rhs;
        self.g /= _rhs;
        self.b /= _rhs;
    }
}
