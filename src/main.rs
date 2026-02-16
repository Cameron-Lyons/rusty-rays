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
use light::{Light, SceneView, TraceConfig, cast_ray};
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

struct CameraBasis {
    pos: Vec3f,
    right: Vec3f,
    up: Vec3f,
    forward: Vec3f,
    tan_fov: f32,
}

struct RenderParams<'a> {
    width: u32,
    height: u32,
    samples: u32,
    camera: CameraBasis,
    scene: SceneView<'a>,
    trace: TraceConfig,
    progress: &'a indicatif::ProgressBar,
}

fn render(params: RenderParams<'_>) -> Vec<Vec3f> {
    let aspect = params.width as f32 / params.height as f32;

    (0..params.height)
        .into_par_iter()
        .flat_map(|j| {
            let row: Vec<Vec3f> = (0..params.width)
                .map(|i| {
                    let mut color = Vec3f(0.0, 0.0, 0.0);

                    if params.samples <= 1 {
                        let x = (2.0 * (i as f32 + 0.5) / params.width as f32 - 1.0)
                            * params.camera.tan_fov
                            * aspect;
                        let y = -(2.0 * (j as f32 + 0.5) / params.height as f32 - 1.0)
                            * params.camera.tan_fov;
                        let dir = (params.camera.right.multiply_scalar(x)
                            + params.camera.up.multiply_scalar(y)
                            + params.camera.forward)
                            .normalize();
                        color = cast_ray(&params.camera.pos, &dir, &params.scene, 0, params.trace);
                    } else {
                        let mut rng = rand::rng();
                        for _ in 0..params.samples {
                            let jx: f32 = rng.random();
                            let jy: f32 = rng.random();
                            let x = (2.0 * (i as f32 + jx) / params.width as f32 - 1.0)
                                * params.camera.tan_fov
                                * aspect;
                            let y = -(2.0 * (j as f32 + jy) / params.height as f32 - 1.0)
                                * params.camera.tan_fov;
                            let dir = (params.camera.right.multiply_scalar(x)
                                + params.camera.up.multiply_scalar(y)
                                + params.camera.forward)
                                .normalize();
                            color = color
                                + cast_ray(
                                    &params.camera.pos,
                                    &dir,
                                    &params.scene,
                                    0,
                                    params.trace,
                                );
                        }
                        color = color.multiply_scalar(1.0 / params.samples as f32);
                    }

                    color
                })
                .collect();
            params.progress.inc(1);
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

    let framebuffer = render(RenderParams {
        width: args.width,
        height: args.height,
        samples: args.samples,
        camera: CameraBasis {
            pos: args.camera_pos,
            right: camera_right,
            up: camera_up,
            forward: camera_forward,
            tan_fov,
        },
        scene: SceneView {
            shapes: &shapes,
            bvh: &bvh,
            lights: &lights,
        },
        trace: TraceConfig {
            max_depth: args.max_depth,
            shadow_samples: args.shadow_samples,
        },
        progress: &progress,
    });

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
