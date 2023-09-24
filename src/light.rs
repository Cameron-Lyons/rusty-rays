#[derive(Clone, Copy, Debug)]
pub struct Light {
    position: Vec3f,
    color: Vec3f, // Using Vec3f for RGB color
    intensity: f32,
}
