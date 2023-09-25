pub struct Lights {
    pub sources: [Vec3f; 3],
}

impl Lights {
    pub fn reflect(&self, I: &Vec3f, N: &Vec3f) -> Vec3f {
        I.subtract(&N.multiply_scalar(2.0 * I.dot(N)))
    }

    pub fn refract(&self, I: &Vec3f, N: &Vec3f, eta_t: f32, eta_i: f32) -> Vec3f {
        let cosi = -f32::max(-1.0, f32::min(1.0, I.dot(N)));
        if cosi < 0.0 {
            return self.refract(I, &N.negate(), eta_i, eta_t);
        }
        let eta = eta_i / eta_t;
        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        if k < 0.0 {
            Vec3f(1.0, 0.0, 0.0)
        } else {
            I.multiply_scalar(eta)
                .add(&N.multiply_scalar(eta * cosi - k.sqrt()))
        }
    }
}
