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
