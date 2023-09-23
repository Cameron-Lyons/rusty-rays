use std::fs::File;
use std::io::prelude::*;

type Vec3f = (f32, f32, f32);

fn render() {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;

    let mut framebuffer: Vec<Vec3f> = vec![(0.0, 0.0, 0.0); WIDTH * HEIGHT];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            framebuffer[i + j * WIDTH] = (j as f32 / HEIGHT as f32, i as f32 / WIDTH as f32, 0.0);
        }
    }
