use std::ops::{Add, Sub, Mul, Neg};

#[derive(Clone, Copy, Debug)]
pub struct Vec3f(pub f32, pub f32, pub f32);

impl Vec3f {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f(x, y, z)
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalized(&self) -> Option<Self> {
        let len = self.length();
        if len == 0.0 {
            None
        } else {
            Some(self.multiply_scalar(1.0 / len))
        }
    }

    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Vec3f(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    #[inline]
    pub fn multiply_scalar(&self, scalar: f32) -> Self {
        Vec3f(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }

    #[inline]
    pub fn multiply(&self, other: &Self) -> Self {
        Vec3f(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Add for Vec3f {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Vec3f(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3f {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Vec3f(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self {
        self.multiply_scalar(scalar)
    }
}

impl Neg for Vec3f {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Vec3f(-self.0, -self.1, -self.2)
    }
}

