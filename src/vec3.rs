
use std::ops::{Add, Sub, Mul, Neg};

pub struct Vec3f(pub f32, pub f32, pub f32);

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f(x, y, z)
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        self.multiply_scalar(1.0 / len)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3f(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn multiply_scalar(&self, scalar: f32) -> Self {
        Vec3f(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Vec3f(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Add for Vec3f {
    type Output = Vec3f;

    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, scalar: f32) -> Vec3f {
        self.multiply_scalar(scalar)
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;

    fn neg(self) -> Vec3f {
        Vec3f(-self.0, -self.1, -self.2)
    }
}

