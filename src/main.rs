use std::{fs::File, io::Write};

use crate::{
    color_util::write_color,
    vec::{Color3},
};

mod color_util;
mod vec;
mod vec_util;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut file = File::create("output.ppm").unwrap();
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    file.write(header.as_bytes()).unwrap();

    for j in 0..image_height {
        print!("\rScanlines remaining: {:04}", image_height - 1 - j);
        std::io::stdout().flush().unwrap();
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = (image_height - 1 - j) as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let color = Color3::from(r, g, b);
            write_color(&mut file, &color);
        }
    }

    println!("\nDone!");
}
