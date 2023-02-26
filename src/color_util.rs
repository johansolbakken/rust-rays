use crate::vec::Color3;
use std::{fs::File, io::Write};

pub fn write_color(file: &mut File, pixel_color: &Color3) {
    let ir = (255.999 * pixel_color.r()) as i32;
    let ig = (255.999 * pixel_color.g()) as i32;
    let ib = (255.999 * pixel_color.b()) as i32;
    let output = format!("{} {} {}\n", ir, ig, ib);
    file.write(output.as_bytes()).unwrap();
}
