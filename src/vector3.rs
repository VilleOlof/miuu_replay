use std::{
    f32::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    #[must_use]
    #[inline(always)]
    pub fn new(values: (f32, f32, f32)) -> Self {
        Self {
            x: values.0,
            y: values.1,
            z: values.2,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let magnitude = self.magnitude();
        Vector3 {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.magnitude().abs() < f32::EPSILON
    }

    pub fn distance_to(&self, other: Vector3) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - other.y * self.z,
            y: -(self.x * other.z - other.x * self.z),
            z: self.x * other.y - other.x * self.y,
        }
    }

    pub fn angle(&self, other: &Vector3) -> f32 {
        (self.dot(other) / self.magnitude() * other.magnitude()).acos()
    }

    pub fn angle_deg(&self, other: &Vector3) -> f32 {
        self.angle(other) * (180.0 / PI)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3::new((self.x * rhs, self.y * rhs, self.z * rhs))
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3::new((self.x / rhs, self.y / rhs, self.z / rhs))
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::new((self.x + rhs.x, self.y + rhs.y, self.z + rhs.z))
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::new((self.x - rhs.x, self.y - rhs.y, self.z - rhs.z))
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(tuple: (f32, f32, f32)) -> Vector3 {
        Vector3::new(tuple)
    }
}

impl PartialEq<Self> for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f32::EPSILON
            && (self.y - other.y).abs() < f32::EPSILON
            && (self.z - other.z).abs() < f32::EPSILON
    }
}
