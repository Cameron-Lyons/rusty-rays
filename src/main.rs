use std::path::PathBuf;

use clap::Parser;
use image::{ImageBuffer, Rgb};
use rand::Rng;
use rayon::prelude::*;

mod bvh;
mod light;
mod material;
mod quartic;
mod shapes;
mod vec3;

use bvh::BvhNode;
use light::{Light, cast_ray};
use material::{CORTEN_STEEL, GLASS, GOLD, IVORY, METAL, MIRROR, RED_RUBBER};
use shapes::{CheckerFloor, Cone, Cube, Cylinder, Shape, Sphere};
use vec3::Vec3f;

#[derive(Parser)]
#[command(name = "rusty-rays")]
#[command(about = "A ray tracer written in Rust")]
struct Args {
    #[arg(short = 'W', long, default_value_t = 1024)]
    width: u32,

    #[arg(short = 'H', long, default_value_t = 768)]
    height: u32,

    #[arg(short, long, default_value_t = 4)]
    samples: u32,

    #[arg(long, default_value_t = 8)]
    shadow_samples: u32,

    #[arg(short, long, default_value = "out.png")]
    output: PathBuf,
}

fn render(
    width: u32,
    height: u32,
    samples: u32,
    shadow_samples: u32,
    shapes: &[Box<dyn Shape>],
    bvh: &BvhNode,
    lights: &[Light],
) -> Vec<Vec3f> {
    let fov = std::f32::consts::FRAC_PI_3;
    let tan_fov = (fov / 2.0).tan();
    let aspect = width as f32 / height as f32;

    (0..(width as usize * height as usize))
        .into_par_iter()
        .map(|idx| {
            let i = idx % width as usize;
            let j = idx / width as usize;
            let mut color = Vec3f(0.0, 0.0, 0.0);

            if samples <= 1 {
                let x = (2.0 * (i as f32 + 0.5) / width as f32 - 1.0) * tan_fov * aspect;
                let y = -(2.0 * (j as f32 + 0.5) / height as f32 - 1.0) * tan_fov;
                let dir = Vec3f(x, y, -1.0).normalize();
                color = cast_ray(
                    &Vec3f(0.0, 0.0, 0.0),
                    &dir,
                    shapes,
                    bvh,
                    lights,
                    0,
                    shadow_samples,
                );
            } else {
                let mut rng = rand::rng();
                for _ in 0..samples {
                    let jx: f32 = rng.random();
                    let jy: f32 = rng.random();
                    let x = (2.0 * (i as f32 + jx) / width as f32 - 1.0) * tan_fov * aspect;
                    let y = -(2.0 * (j as f32 + jy) / height as f32 - 1.0) * tan_fov;
                    let dir = Vec3f(x, y, -1.0).normalize();
                    color = color
                        + cast_ray(
                            &Vec3f(0.0, 0.0, 0.0),
                            &dir,
                            shapes,
                            bvh,
                            lights,
                            0,
                            shadow_samples,
                        );
                }
                color = color.multiply_scalar(1.0 / samples as f32);
            }

            color
        })
        .collect()
}

fn main() {
    let args = Args::parse();

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Sphere::new(Vec3f(-3.0, 0.0, -16.0), 2.0, IVORY)),
        Box::new(Sphere::new(Vec3f(-1.0, -1.5, -12.0), 2.0, GLASS)),
        Box::new(Sphere::new(Vec3f(1.5, -0.5, -18.0), 3.0, RED_RUBBER)),
        Box::new(Sphere::new(Vec3f(7.0, 5.0, -18.0), 4.0, MIRROR)),
        Box::new(Cube::new(Vec3f(5.0, -2.75, -14.0), 2.5, METAL)),
        Box::new(Cylinder::new(Vec3f(-6.0, -4.0, -20.0), 4.0, 1.2, GOLD)),
        Box::new(Cone::new(Vec3f(9.0, -4.0, -22.0), 5.0, 2.0, CORTEN_STEEL)),
        Box::new(CheckerFloor::new(-4.0, (-15.0, 15.0), (-35.0, -5.0))),
    ];

    let mut indices: Vec<usize> = (0..shapes.len()).collect();
    let bvh = BvhNode::build(&shapes, &mut indices);

    let lights = vec![
        Light {
            position: Vec3f(-20.0, 20.0, 20.0),
            intensity: 1.5,
            radius: 1.0,
        },
        Light {
            position: Vec3f(30.0, 50.0, -25.0),
            intensity: 1.8,
            radius: 1.5,
        },
        Light {
            position: Vec3f(30.0, 20.0, 30.0),
            intensity: 1.7,
            radius: 1.0,
        },
    ];

    let framebuffer = render(
        args.width,
        args.height,
        args.samples,
        args.shadow_samples,
        &shapes,
        &bvh,
        &lights,
    );

    let img = ImageBuffer::from_fn(args.width, args.height, |x, y| {
        let Vec3f(r, g, b) = framebuffer[(y * args.width + x) as usize];
        let mx = r.max(g).max(b);
        let scale = if mx > 1.0 { 1.0 / mx } else { 1.0 };
        Rgb([
            (255.0 * (r * scale).clamp(0.0, 1.0)) as u8,
            (255.0 * (g * scale).clamp(0.0, 1.0)) as u8,
            (255.0 * (b * scale).clamp(0.0, 1.0)) as u8,
        ])
    });

    img.save(&args.output).expect("Failed to save image");
    eprintln!("Rendered to {}", args.output.display());
}
