use std::fs::File;
use std::io::{self, prelude::*};

mod vec3;

fn render(width: usize, height: usize) -> io::Result<()> {
    let mut framebuffer: Vec<vec3::Vec3f> = vec![vec3::Vec3f(0.0, 0.0, 0.0); width * height];

    for j in 0..height {
        for i in 0..width {
            framebuffer[i + j * width] =
                vec3::Vec3f(j as f32 / height as f32, i as f32 / width as f32, 0.0);
        }
    }

    let mut ofs = File::create("./out.ppm")?;
    ofs.write_all(format!("P6\n{} {}\n255\n", width, height).as_bytes())?;

    for &vec3::Vec3f(r, g, b) in &framebuffer {
        let max_value = 255.0;
        ofs.write_all(&[
            (max_value * r.clamp(0.0, 1.0)) as u8,
            (max_value * g.clamp(0.0, 1.0)) as u8,
            (max_value * b.clamp(0.0, 1.0)) as u8,
        ])?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = render(1024, 768) {
        eprintln!("Error: {}", e);
    }
}
