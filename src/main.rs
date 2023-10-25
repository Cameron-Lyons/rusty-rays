use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

mod vec3;
use vec3::Vec3f;

fn create_gradient_image(width: usize, height: usize) -> Vec<Vec3f> {
    let mut framebuffer = Vec::with_capacity(width * height);
    for j in 0..height {
        for i in 0..width {
            framebuffer.push(Vec3f(j as f32 / height as f32, i as f32 / width as f32, 0.0));
        }
    }
    framebuffer
}

fn render(width: usize, height: usize, path: &Path) -> io::Result<()> {
    let framebuffer = create_gradient_image(width, height);
    let mut file = File::create(path)?;

    writeln!(file, "P6\n{} {}\n255", width, height)?;
    for Vec3f(r, g, b) in framebuffer {
        let max_value = 255.0;
        file.write_all(&[
            (max_value * r.clamp(0.0, 1.0)) as u8,
            (max_value * g.clamp(0.0, 1.0)) as u8,
            (max_value * b.clamp(0.0, 1.0)) as u8,
        ])?;
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    render(1024, 768, Path::new("out.ppm"))?;
    Ok(())
}

