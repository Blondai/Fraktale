use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::settings::Float;

#[derive(Clone, Copy)]
pub struct Complex {
    real: Float,
    imaginary: Float,
}

// Display
impl Display for Complex {
    fn fmt(&self, format: &mut Formatter<'_>) -> std::fmt::Result {
        write!(format, "{} + {}i", self.real, self.imaginary)
    }
}

// New
impl Complex {
    pub fn new(real: Float, imaginary: Float) -> Complex {
        Complex { real, imaginary }
    }

    pub const fn const_new(real: Float, imaginary: Float) -> Complex {
        Complex { real, imaginary }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self: Complex, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

impl Add<f32> for Complex {
    type Output = Complex;

    fn add(self: Complex, other: Float) -> Complex {
        Complex {
            real: self.real + other,
            imaginary: self.imaginary,
        }
    }
}

impl Add<Complex> for f32 {
    type Output = Complex;

    fn add(self: Float, other: Complex) -> Complex {
        Complex {
            real: self + other.real,
            imaginary: other.imaginary,
        }
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self: Complex) -> Complex {
        Complex {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self: Complex, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }
}

impl Sub<Float> for Complex {
    type Output = Complex;

    fn sub(self: Complex, other: Float) -> Complex {
        Complex {
            real: self.real - other,
            imaginary: self.imaginary,
        }
    }
}

impl Sub<Complex> for Float {
    type Output = Complex;

    fn sub(self: Float, other: Complex) -> Complex {
        Complex {
            real: self - other.real,
            imaginary: -other.imaginary,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self: Complex, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }
}

impl Mul<Float> for Complex {
    type Output = Complex;

    fn mul(self: Complex, other: Float) -> Complex {
        Complex {
            real: self.real * other,
            imaginary: self.imaginary * other,
        }
    }
}

impl Mul<Complex> for Float {
    type Output = Complex;

    fn mul(self: Float, other: Complex) -> Complex {
        Complex {
            real: self * other.real,
            imaginary: self * other.imaginary,
        }
    }
}

// Abs
impl Complex {
    fn abs_sq(self: Complex) -> Float {
        self.real.powi(2i32) + self.imaginary.powi(2i32)
    }

    fn abs(self: Complex) -> Float {
        if self.real == 0 as Float {
            self.imaginary
        } else if self.imaginary == 0 as Float {
            self.real
        } else {
            self.abs_sq().sqrt()
        }
    }

    fn taxi(self: Complex) -> Float {
        self.real.abs() + self.imaginary.abs()
    }

    pub fn len(self: Complex) -> Float {
        self.abs()
    }
}

// Inverse
impl Complex {
    fn inverse(self: Complex) -> Complex {
        let abs_sq: Float = self.real.powi(2i32) + self.imaginary.powi(2i32);
        Complex {
            real: self.real / abs_sq,
            imaginary: -self.imaginary / abs_sq,
        }
    }

    pub fn inv(self: Complex) -> Complex {
        self.inverse()
    }
}

// Div
impl Div for Complex {
    type Output = Complex;

    fn div(self: Complex, other: Complex) -> Complex {
        let abs_sq: Float = other.real.powi(2i32) + other.imaginary.powi(2i32);
        Complex {
            real: (self.real * other.real + self.imaginary * other.imaginary) / abs_sq,
            imaginary: (self.imaginary * other.real - self.real * other.imaginary) / abs_sq,
        }
    }
}

impl Div<Float> for Complex {
    type Output = Complex;

    fn div(self: Complex, other: Float) -> Complex {
        Complex {
            real: self.real / other,
            imaginary: self.imaginary / other,
        }
    }
}

impl Div<Complex> for Float {
    type Output = Complex;

    fn div(self: Float, other: Complex) -> Complex {
        let abs_sq: Float = other.real.powi(2i32) + other.imaginary.powi(2i32);
                Complex {
            real: (other.real * self) / abs_sq,
            imaginary: - (other.imaginary * self) / abs_sq,
        }
    }
}

// Power
impl Complex {
    pub fn pow(self: Complex, power: u32) -> Complex {
        let mut new_complex: Complex = Complex::one();
        for _ in 0..power as usize {
            new_complex = new_complex * self;
        }
        new_complex
    }
}

// Instances
impl Complex {
    fn one() -> Complex {
        Complex::new(1 as Float, 0 as Float)
    }

    fn zero() -> Complex {
        Complex::new(0 as Float, 0 as Float)
    }

    fn i() -> Complex {
        Complex::new(0 as Float, 1 as Float)
    }
}