use crate::tracer::Vector3f;
use cgmath::*;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn from_vec3f(v: Vector3f) -> Self {
        Self::new(v.x, v.y, v.z)
    }

    pub fn to_vec3f(self) -> Vector3f {
        vec3(self.red, self.green, self.blue)
    }

    #[must_use]
    pub fn sqrt(self) -> Self {
        Self::new(self.red.sqrt(), self.green.sqrt(), self.blue.sqrt())
    }
}

impl Mul<Vector3f> for Color {
    type Output = Color;

    fn mul(self, v: Vector3f) -> Color {
        Color::new(self.red * v.x, self.green * v.y, self.blue * v.z)
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, c: Color) -> Color {
        Color::new(self.red * c.red, self.green * c.green, self.blue * c.blue)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, f: f64) -> Color {
        Color::new(self.red * f, self.green * f, self.blue * f)
    }
}

impl Add<Vector3f> for Color {
    type Output = Color;

    fn add(self, v: Vector3f) -> Color {
        Color::new(self.red + v.x, self.green + v.y, self.blue + v.z)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, c: Color) -> Color {
        Color::new(self.red + c.red, self.green + c.green, self.blue + c.blue)
    }
}

impl Add<f64> for Color {
    type Output = Color;

    fn add(self, f: f64) -> Color {
        Color::new(self.red + f, self.green + f, self.blue + f)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, f: f64) {
        self.red = self.red * f;
        self.green = self.green * f;
        self.blue = self.blue * f;
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        };
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, f: f64) {
        *self = Self {
            red: self.red / f,
            green: self.green / f,
            blue: self.blue / f,
        };
    }
}

impl DivAssign<u64> for Color {
    fn div_assign(&mut self, rhs: u64) {
        let f = rhs as f64;
        *self = Self {
            red: self.red / f,
            green: self.green / f,
            blue: self.blue / f,
        };
    }
}

impl DivAssign<u32> for Color {
    fn div_assign(&mut self, rhs: u32) {
        let f = rhs as f64;
        *self = Self {
            red: self.red / f,
            green: self.green / f,
            blue: self.blue / f,
        };
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, f: f64) -> Color {
        Color::new(self.red / f, self.green + f, self.blue + f)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Color#{:0>2x}{:0>2x}{:0>2x})",
            (self.red * 255f64) as u8,
            (self.green * 255f64) as u8,
            (self.blue * 255f64) as u8
        )
    }
}
