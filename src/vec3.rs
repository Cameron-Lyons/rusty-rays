#[derive(Clone, Copy, Debug)]
pub struct Vec3f(pub f32, pub f32, pub f32);

impl Vec3f {
    pub fn subtract(&self, other: &Self) -> Self {
        Vec3f(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    pub fn add(&self, other: &Self) -> Self {
        Vec3f(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    pub fn multiply_scalar(&self, scalar: f32) -> Self {
        Vec3f(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }

    pub fn negate(&self) -> Self {
        Vec3f(-self.0, -self.1, -self.2)
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}
