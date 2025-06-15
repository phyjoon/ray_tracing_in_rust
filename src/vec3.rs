use std::{
    fmt::{Display, Formatter},
    ops::{Add, Div, Mul, Neg, Sub},
};

// Section 3 of `RayTracingInOneWeekend`
// Represents a 3D vector with x, y, and z components.
#[derive(Debug, Clone, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
    pub fn length(&self) -> f64 {
        Vec3::dot(&self, &self)
    }
    pub fn length_squared(&self) -> f64 {
        Vec3::dot(&self, &self).sqrt()
    }
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
    pub fn unit_vector(&self) -> Vec3 {
        let len = self.length();
        if len == 0.0 {
            self.clone()
        } else {
            self.clone() / len
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, t: f64) -> Vec3 {
        Vec3(self.0 + t, self.1 + t, self.2 + t)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        Vec3(self.0 / t, self.1 / t, self.2 / t)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}
