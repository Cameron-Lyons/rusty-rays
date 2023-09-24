pub struct Lights {
    pub sources: [Vec3f; 3],
}

impl Lights {
    pub fn reflect(&self, I: &Vec3f, N: &Vec3f) -> Vec3f {
        I.subtract(&N.multiply_scalar(2.0 * I.dot(N)))
    }
