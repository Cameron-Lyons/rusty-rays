# rusty-rays

A CPU ray tracer written in Rust. Renders 3D scenes with reflection, refraction, shadows, and a checkerboard floor plane. Outputs images in PPM format.

## Features

- Phong lighting model with diffuse and specular components
- Recursive reflection and refraction (up to depth 4)
- Shadow casting from multiple light sources
- Predefined materials: ivory, glass, red rubber, mirror, metal, gold, marble, velvet, and more
- Shape primitives: sphere, cube, cone, cylinder, pyramid, ovoid, torus, and rectangular prism
- Quartic/cubic/quadratic equation solvers for analytic ray-shape intersections
- Parallel rendering with [rayon](https://github.com/rayon-rs/rayon)

## Usage

```fish
cargo run
```

This renders the default scene (1024x768) to `out.ppm`.

## Building

```fish
cargo build --release
```

## Testing

```fish
cargo test
```

## Project Structure

```
src/
  main.rs      - scene setup, rendering loop, PPM output
  vec3.rs      - 3D vector type with arithmetic operations
  shapes.rs    - ray-intersectable shape primitives
  light.rs     - lighting, reflection, refraction, shadow casting
  material.rs  - material definitions (albedo, color, specularity, refraction)
  quartic.rs   - polynomial equation solvers
```
