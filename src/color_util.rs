use crate::{math::clamp, vec::Color3};
use std::{fs::File, io::Write};

pub fn write_color(file: &mut File, pixel_color: &Color3, samples_per_pixel: u32) {
    let mut r = pixel_color.r();
    let mut g = pixel_color.g();
    let mut b = pixel_color.b();

    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;

    // Write the translated [0, 255] value of each color component
    let output = format!("{} {} {}\n", ir, ig, ib);
    file.write(output.as_bytes()).unwrap();
}
