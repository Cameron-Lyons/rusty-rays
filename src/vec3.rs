#![allow(dead_code)]

use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
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

    #[inline]
    pub fn norm(&self) -> f32 {
        self.length()
    }

    #[inline]
    pub fn magnitude_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn normalized(&self) -> Option<Self> {
        let len = self.length();
        if len == 0.0 {
            None
        } else {
            Some(self.multiply_scalar(1.0 / len))
        }
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            *self
        } else {
            self.multiply_scalar(1.0 / len)
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

    #[inline]
    pub fn subtract(&self, other: &Self) -> Self {
        Vec3f(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    #[inline]
    pub fn negate(&self) -> Self {
        Vec3f(-self.0, -self.1, -self.2)
    }

    #[inline]
    pub fn add_ref(&self, other: &Self) -> Self {
        Vec3f(self.0 + other.0, self.1 + other.1, self.2 + other.2)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vec3f(1.0, 2.0, 3.0);
        let b = Vec3f(4.0, 5.0, 6.0);
        assert_eq!(a + b, Vec3f(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_add_ref() {
        let a = Vec3f(1.0, 2.0, 3.0);
        let b = Vec3f(4.0, 5.0, 6.0);
        assert_eq!(a.add_ref(&b), Vec3f(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_subtract() {
        let a = Vec3f(4.0, 5.0, 6.0);
        let b = Vec3f(1.0, 2.0, 3.0);
        assert_eq!(a.subtract(&b), Vec3f(3.0, 3.0, 3.0));
        assert_eq!(a - b, Vec3f(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_dot() {
        let a = Vec3f(1.0, 2.0, 3.0);
        let b = Vec3f(4.0, 5.0, 6.0);
        assert_eq!(a.dot(&b), 32.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3f(1.0, 0.0, 0.0);
        let b = Vec3f(0.0, 1.0, 0.0);
        assert_eq!(a.cross(&b), Vec3f(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_length() {
        let v = Vec3f(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);
        assert_eq!(v.norm(), 5.0);
    }

    #[test]
    fn test_normalize() {
        let v = Vec3f(3.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Vec3f(1.0, 0.0, 0.0));
        assert_eq!(v.normalized(), Some(Vec3f(1.0, 0.0, 0.0)));
    }

    #[test]
    fn test_negate() {
        let v = Vec3f(1.0, -2.0, 3.0);
        assert_eq!(v.negate(), Vec3f(-1.0, 2.0, -3.0));
        assert_eq!(-v, Vec3f(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_multiply_scalar() {
        let v = Vec3f(1.0, 2.0, 3.0);
        assert_eq!(v.multiply_scalar(2.0), Vec3f(2.0, 4.0, 6.0));
        assert_eq!(v * 2.0, Vec3f(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_magnitude_squared() {
        let v = Vec3f(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude_squared(), 14.0);
    }
}
