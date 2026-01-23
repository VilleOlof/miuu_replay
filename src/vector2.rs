use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(values: (f32, f32)) -> Self {
        Self {
            x: values.0,
            y: values.1,
        }
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y * self.y))
    }

    pub fn sqr_magnitude(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        if magnitude > f32::EPSILON {
            *self = *self / magnitude;
        } else {
            *self = Vector2::new((0.0, 0.0));
        }
    }

    pub fn distance_to(&self, other: &Vector2) -> f32 {
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;
        f32::sqrt((diff_x * diff_x) + (diff_y * diff_y))
    }

    pub fn dot(&self, rhs: &Vector2) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    pub fn angle(&self, other: &Vector2) -> f32 {
        const K_EPSILON_NORMAL_SQRT: f32 = 1e-15;
        let denominator = f32::sqrt(self.sqr_magnitude() * other.sqr_magnitude());
        if denominator < K_EPSILON_NORMAL_SQRT {
            0.0
        } else {
            let dot = f32::clamp(Self::dot(self, other), -1.0, 1.0);
            f32::to_degrees(f32::acos(dot))
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2::new((self.x * rhs, self.y * rhs))
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        Vector2::new((self.x / rhs, self.y / rhs))
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2::new((self.x + rhs.x, self.y + rhs.y))
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2::new((self.x - rhs.x, self.y - rhs.y))
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(tuple: (f32, f32)) -> Vector2 {
        Vector2::new(tuple)
    }
}

impl PartialEq<Self> for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f32::EPSILON && (self.y - other.y).abs() < f32::EPSILON
    }
}
