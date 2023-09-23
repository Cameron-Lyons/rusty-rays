#[derive(Clone, Copy, Debug)]
struct Vec3f(f32, f32, f32);

impl Vec3f {
    fn subtract(&self, other: &Vec3f) -> Vec3f {
        Vec3f(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    fn dot(&self, other: &Vec3f) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    fn magnitude_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
}

struct Sphere {
    center: Vec3f,
    radius: f32,
}

impl Sphere {
    fn new(center: Vec3f, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
