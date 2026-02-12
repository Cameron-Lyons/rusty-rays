use std::path::PathBuf;

use clap::Parser;
use image::{ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use rand::RngExt;
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

fn parse_vec3(s: &str) -> Result<Vec3f, String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return Err(format!("expected 3 comma-separated floats, got '{s}'"));
    }
    let x = parts[0].trim().parse::<f32>().map_err(|e| e.to_string())?;
    let y = parts[1].trim().parse::<f32>().map_err(|e| e.to_string())?;
    let z = parts[2].trim().parse::<f32>().map_err(|e| e.to_string())?;
    Ok(Vec3f(x, y, z))
}

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

    #[arg(short = 'd', long, default_value_t = 4)]
    max_depth: i32,

    #[arg(long, default_value = "0,0,0", value_parser = parse_vec3)]
    camera_pos: Vec3f,

    #[arg(long, default_value = "0,0,-16", value_parser = parse_vec3)]
    look_at: Vec3f,

    #[arg(long, default_value_t = 60.0)]
    fov: f32,

    #[arg(short, long, default_value = "out.png")]
    output: PathBuf,
}

fn render(
    width: u32,
    height: u32,
    samples: u32,
    shadow_samples: u32,
    max_depth: i32,
    camera_pos: Vec3f,
    camera_right: Vec3f,
    camera_up: Vec3f,
    camera_forward: Vec3f,
    tan_fov: f32,
    shapes: &[Box<dyn Shape>],
    bvh: &BvhNode,
    lights: &[Light],
    progress: &indicatif::ProgressBar,
) -> Vec<Vec3f> {
    let aspect = width as f32 / height as f32;

    (0..height)
        .into_par_iter()
        .flat_map(|j| {
            let row: Vec<Vec3f> = (0..width)
                .map(|i| {
                    let mut color = Vec3f(0.0, 0.0, 0.0);

                    if samples <= 1 {
                        let x = (2.0 * (i as f32 + 0.5) / width as f32 - 1.0) * tan_fov * aspect;
                        let y = -(2.0 * (j as f32 + 0.5) / height as f32 - 1.0) * tan_fov;
                        let dir = (camera_right.multiply_scalar(x)
                            + camera_up.multiply_scalar(y)
                            + camera_forward)
                            .normalize();
                        color = cast_ray(
                            &camera_pos,
                            &dir,
                            shapes,
                            bvh,
                            lights,
                            0,
                            max_depth,
                            shadow_samples,
                        );
                    } else {
                        let mut rng = rand::rng();
                        for _ in 0..samples {
                            let jx: f32 = rng.random();
                            let jy: f32 = rng.random();
                            let x = (2.0 * (i as f32 + jx) / width as f32 - 1.0) * tan_fov * aspect;
                            let y = -(2.0 * (j as f32 + jy) / height as f32 - 1.0) * tan_fov;
                            let dir = (camera_right.multiply_scalar(x)
                                + camera_up.multiply_scalar(y)
                                + camera_forward)
                                .normalize();
                            color = color
                                + cast_ray(
                                    &camera_pos,
                                    &dir,
                                    shapes,
                                    bvh,
                                    lights,
                                    0,
                                    max_depth,
                                    shadow_samples,
                                );
                        }
                        color = color.multiply_scalar(1.0 / samples as f32);
                    }

                    color
                })
                .collect();
            progress.inc(1);
            row
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

    let fov_rad = args.fov.to_radians();
    let tan_fov = (fov_rad / 2.0).tan();

    let world_up = Vec3f(0.0, 1.0, 0.0);
    let camera_forward = args.look_at.subtract(&args.camera_pos).normalize();
    let camera_right = camera_forward.cross(&world_up).normalize();
    let camera_up = camera_right.cross(&camera_forward);

    let progress = ProgressBar::new(args.height as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{bar:40.cyan/blue} {pos}/{len} scanlines [{elapsed_precise} elapsed, {eta_precise} remaining]")
            .unwrap()
            .progress_chars("##-"),
    );

    let framebuffer = render(
        args.width,
        args.height,
        args.samples,
        args.shadow_samples,
        args.max_depth,
        args.camera_pos,
        camera_right,
        camera_up,
        camera_forward,
        tan_fov,
        &shapes,
        &bvh,
        &lights,
        &progress,
    );

    progress.finish_and_clear();

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
