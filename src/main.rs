use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use rayon::prelude::*;

mod light;
mod material;
mod quartic;
mod shapes;
mod vec3;

use light::{Light, cast_ray};
use material::{GLASS, IVORY, MIRROR, RED_RUBBER};
use shapes::Sphere;
use vec3::Vec3f;

fn render(
    width: usize,
    height: usize,
    spheres: &[Sphere],
    lights: &[Light],
    path: &Path,
) -> io::Result<()> {
    let fov = std::f32::consts::FRAC_PI_3;
    let tan_fov = (fov / 2.0).tan();

    let framebuffer: Vec<Vec3f> = (0..width * height)
        .into_par_iter()
        .map(|idx| {
            let i = idx % width;
            let j = idx / width;
            let x = (2.0 * (i as f32 + 0.5) / width as f32 - 1.0) * tan_fov * width as f32
                / height as f32;
            let y = -(2.0 * (j as f32 + 0.5) / height as f32 - 1.0) * tan_fov;
            let dir = Vec3f(x, y, -1.0).normalize();
            cast_ray(&Vec3f(0.0, 0.0, 0.0), &dir, spheres, lights, 0)
        })
        .collect();

    let mut file = File::create(path)?;
    writeln!(file, "P6\n{} {}\n255", width, height)?;
    for Vec3f(r, g, b) in &framebuffer {
        let mx = r.max(*g).max(*b);
        let scale = if mx > 1.0 { 1.0 / mx } else { 1.0 };
        file.write_all(&[
            (255.0 * (r * scale).clamp(0.0, 1.0)) as u8,
            (255.0 * (g * scale).clamp(0.0, 1.0)) as u8,
            (255.0 * (b * scale).clamp(0.0, 1.0)) as u8,
        ])?;
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let spheres = vec![
        Sphere::new(Vec3f(-3.0, 0.0, -16.0), 2.0, IVORY),
        Sphere::new(Vec3f(-1.0, -1.5, -12.0), 2.0, GLASS),
        Sphere::new(Vec3f(1.5, -0.5, -18.0), 3.0, RED_RUBBER),
        Sphere::new(Vec3f(7.0, 5.0, -18.0), 4.0, MIRROR),
    ];

    let lights = vec![
        Light {
            position: Vec3f(-20.0, 20.0, 20.0),
            intensity: 1.5,
        },
        Light {
            position: Vec3f(30.0, 50.0, -25.0),
            intensity: 1.8,
        },
        Light {
            position: Vec3f(30.0, 20.0, 30.0),
            intensity: 1.7,
        },
    ];

    render(1024, 768, &spheres, &lights, Path::new("out.ppm"))?;
    Ok(())
}
