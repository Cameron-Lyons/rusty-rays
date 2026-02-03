#![allow(dead_code)]

const EPSILON: f32 = 1e-12;

pub fn solve_quartic(coeffs: &[f32; 5]) -> Vec<f32> {
    let [a, b, c, d, e] = *coeffs;

    if a.abs() < EPSILON {
        return solve_cubic(&[b, c, d, e]);
    }

    let b = b / a;
    let c = c / a;
    let d = d / a;
    let e = e / a;

    let sq = b * b;

    let p = -3.0 / 8.0 * sq + c;
    let q = 1.0 / 8.0 * sq * b - 0.5 * b * c + d;
    let r = -3.0 / 256.0 * sq * sq + c * sq / 16.0 - 1.0 / 4.0 * b * d + e;

    if r.abs() < EPSILON {
        let mut roots = solve_cubic(&[1.0, 0.0, p, q]);
        for root in &mut roots {
            *root -= b / 4.0;
        }
        return roots;
    }

    let cubic_coeffs = [1.0, -p / 2.0, -r, r * p / 2.0 - q * q / 8.0];

    let cubic_roots = solve_cubic(&cubic_coeffs);
    let z = cubic_roots
        .iter()
        .copied()
        .find(|&r| 2.0 * r - p > EPSILON)
        .unwrap_or(cubic_roots.first().copied().unwrap_or(0.0));

    let u = (2.0 * z - p).max(0.0).sqrt();

    let mut roots = vec![];

    let v1 = z * z - r;
    let s1 = if u.abs() < EPSILON {
        v1.max(0.0).sqrt()
    } else {
        q / (2.0 * u)
    };

    let disc1 = u * u - 4.0 * (z - s1);
    if disc1 >= -EPSILON {
        let disc1 = disc1.max(0.0).sqrt();
        roots.push((-u + disc1) / 2.0 - b / 4.0);
        roots.push((-u - disc1) / 2.0 - b / 4.0);
    }

    let disc2 = u * u - 4.0 * (z + s1);
    if disc2 >= -EPSILON {
        let disc2 = disc2.max(0.0).sqrt();
        roots.push((u + disc2) / 2.0 - b / 4.0);
        roots.push((u - disc2) / 2.0 - b / 4.0);
    }

    roots
}

pub fn solve_cubic(coeffs: &[f32; 4]) -> Vec<f32> {
    let [a, b, c, d] = *coeffs;

    if a.abs() < EPSILON {
        return solve_quadratic(&[b, c, d]);
    }

    let b = b / a;
    let c = c / a;
    let d = d / a;

    let p = c - b * b / 3.0;
    let q = 2.0 * b * b * b / 27.0 - b * c / 3.0 + d;
    let discriminant = q * q / 4.0 + p * p * p / 27.0;

    let shift = -b / 3.0;

    if discriminant > EPSILON {
        let sd = discriminant.sqrt();
        let u = cbrt(-q / 2.0 + sd);
        let v = cbrt(-q / 2.0 - sd);
        vec![u + v + shift]
    } else if discriminant.abs() <= EPSILON {
        let u = cbrt(-q / 2.0);
        let r1 = 2.0 * u + shift;
        let r2 = -u + shift;
        if (r1 - r2).abs() < 1e-6 {
            vec![r1]
        } else {
            vec![r1, r2]
        }
    } else {
        let r = (-p * p * p / 27.0).sqrt();
        let phi = (-q / (2.0 * r)).clamp(-1.0, 1.0).acos();
        let cube_root_r = r.powf(1.0 / 3.0);
        vec![
            2.0 * cube_root_r * (phi / 3.0).cos() + shift,
            2.0 * cube_root_r * ((phi + 2.0 * std::f32::consts::PI) / 3.0).cos() + shift,
            2.0 * cube_root_r * ((phi + 4.0 * std::f32::consts::PI) / 3.0).cos() + shift,
        ]
    }
}

pub fn solve_quadratic(coeffs: &[f32; 3]) -> Vec<f32> {
    let (a, b, c) = (coeffs[0], coeffs[1], coeffs[2]);

    if a.abs() < EPSILON {
        if b.abs() < EPSILON {
            return vec![];
        }
        return vec![-c / b];
    }

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < -EPSILON {
        vec![]
    } else if discriminant.abs() <= EPSILON {
        vec![-b / (2.0 * a)]
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let denominator = 2.0 * a;
        vec![
            (-b + sqrt_discriminant) / denominator,
            (-b - sqrt_discriminant) / denominator,
        ]
    }
}

fn cbrt(x: f32) -> f32 {
    x.signum() * x.abs().powf(1.0 / 3.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-3
    }

    fn contains_root(roots: &[f32], expected: f32) -> bool {
        roots.iter().any(|&r| approx_eq(r, expected))
    }

    #[test]
    fn test_quadratic_two_roots() {
        let roots = solve_quadratic(&[1.0, -3.0, 2.0]);
        assert_eq!(roots.len(), 2);
        assert!(contains_root(&roots, 1.0));
        assert!(contains_root(&roots, 2.0));
    }

    #[test]
    fn test_quadratic_one_root() {
        let roots = solve_quadratic(&[1.0, -2.0, 1.0]);
        assert_eq!(roots.len(), 1);
        assert!(approx_eq(roots[0], 1.0));
    }

    #[test]
    fn test_quadratic_no_roots() {
        let roots = solve_quadratic(&[1.0, 0.0, 1.0]);
        assert!(roots.is_empty());
    }

    #[test]
    fn test_quadratic_degenerate_linear() {
        let roots = solve_quadratic(&[0.0, 2.0, -4.0]);
        assert_eq!(roots.len(), 1);
        assert!(approx_eq(roots[0], 2.0));
    }

    #[test]
    fn test_cubic_one_root() {
        let roots = solve_cubic(&[1.0, 0.0, 0.0, -8.0]);
        assert!(contains_root(&roots, 2.0));
    }

    #[test]
    fn test_cubic_three_roots() {
        let roots = solve_cubic(&[1.0, -6.0, 11.0, -6.0]);
        assert!(roots.len() >= 3);
        assert!(contains_root(&roots, 1.0));
        assert!(contains_root(&roots, 2.0));
        assert!(contains_root(&roots, 3.0));
    }

    #[test]
    fn test_quartic_known_roots() {
        let roots = solve_quartic(&[1.0, -10.0, 35.0, -50.0, 24.0]);
        assert!(contains_root(&roots, 1.0));
        assert!(contains_root(&roots, 2.0));
        assert!(contains_root(&roots, 3.0));
        assert!(contains_root(&roots, 4.0));
    }
}
