fn solve_quartic(coeffs: &[f32; 5]) -> Vec<f32> {
    let a = coeffs[0];
    let b = coeffs[1] / a;
    let c = coeffs[2] / a;
    let d = coeffs[3] / a;
    let e = coeffs[4] / a;

    let sq = b * b;

    let p = -3.0 / 8.0 * sq + c;
    let q = 1.0 / 8.0 * sq * b - 0.5 * b * c + d;
    let r = -3.0 / 256.0 * sq * sq + c * sq / 16.0 - 1.0 / 4.0 * b * d + e;

    if r.abs() < 1e-12 {
        return solve_cubic(&[a, c, d, e]);
    }

    let cubic_coeffs = [
        1.0,
        0.5 * p,
        -r,
        -0.25 * q * q,
    ];

    let z = solve_cubic(&cubic_coeffs).into_iter().next().unwrap_or(0.0);

    let d1 = 2.0 * z - p;
    let d2 = if d1.abs() < 1e-12 {
        -q / sqrt(2.0 * z)
    } else {
        q / (2.0 * z)
    };

    let quadratic1 = [
        1.0,
        -z.sqrt(),
        z - d2,
    ];

    let quadratic2 = [
        1.0,
        z.sqrt(),
        z + d2,
    ];

    let mut roots = vec![];

    roots.extend(solve_quadratic(&quadratic1));
    roots.extend(solve_quadratic(&quadratic2));

    roots
}

fn solve_cubic(coeffs: &[f32; 4]) -> Vec<f32> {
    let a = coeffs[0];
    let b = coeffs[1] / a;
    let c = coeffs[2] / a;
    let d = coeffs[3] / a;

    let delta_0 = c * c - 3.0 * b * d + 12.0 * a * e;
    let delta_1 = 2.0 * c * c * c - 9.0 * b * c * d + 27.0 * a * d * d + 27.0 * b * b * e - 72.0 * a * c * e;

    let discriminant = 18.0 * a * b * c * d - 4.0 * b.powi(3) * d + b.powi(2) * c.powi(2) - 4.0 * a * c.powi(3) - 27.0 * a.powi(2) * d.powi(2);

    let mut roots = Vec::new();

    if discriminant > 0.0 {
        // 3 real roots
        let sd = discriminant.sqrt();
        let u = cbrt(-q / 2.0 + sd);
        let v = cbrt(-q / 2.0 - sd);

        roots.push(u + v - b / (3.0 * a));
    } else if discriminant == 0.0 {
        // 2 real roots
        roots.push( ... );  // Logic to compute the double root
        roots.push( ... );  // Logic to compute the single root
    } else {
        // 1 real root
        let c_cube_root = ((delta_1 + (delta_1 * delta_1 - 4.0 * delta_0 * delta_0 * delta_0).sqrt()).powf(1.0/3.0)) / (3.0f32.cbrt() * 2.0f32.powf(1.0/3.0));
        roots.push((-1.0/(3.0*a))*(b + c_cube_root + delta_0/c_cube_root));
    }

    roots
}


fn solve_quadratic(coeffs: &[f32; 3]) -> Vec<f32> {
    let a = coeffs[0];
    let b = coeffs[1] / a;
    let c = coeffs[2] / a;

    let discriminant = b * b - 4.0 * c;

    if discriminant < 0.0 {
        vec![]
    } else if discriminant.abs() < 1e-12 {
        vec![-0.5 * b]
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        vec![
            -0.5 * (b + sqrt_discriminant),
            -0.5 * (b - sqrt_discriminant),
        ]
    }
}

