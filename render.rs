use std::fs::File;
use std::io::prelude::*;

type Vec3f = (f32, f32, f32);

fn render(width: usize, height: usize) {
    let mut framebuffer: Vec<Vec3f> = vec![(0.0, 0.0, 0.0); width * height];

    for j in 0..height {
        for i in 0..width {
            framebuffer[i + j * width] = (j as f32 / height as f32, i as f32 / width as f32, 0.0);
        }
    }

    let mut ofs = File::create("./out.ppm").expect("Unable to create file");
    ofs.write_all(format!("P6\n{} {}\n255\n", width, height).as_bytes())
        .expect("Failed to write to file");

    for &(r, g, b) in &framebuffer {
        let max_value = 255.0;
        ofs.write_all(&[
            (max_value * r.clamp(0.0, 1.0)) as u8,
            (max_value * g.clamp(0.0, 1.0)) as u8,
            (max_value * b.clamp(0.0, 1.0)) as u8,
        ])
        .expect("Failed to write to file");
    }
}
