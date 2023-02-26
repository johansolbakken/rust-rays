use std::{fs::File, io::Write};

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut file = File::create("output.ppm").unwrap();
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    file.write(header.as_bytes()).unwrap();

    for j in 0 ..image_height {
        print!("\rScanlines remaining: {:04}", image_height - 1 - j);
        std::io::stdout().flush().unwrap();
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = (image_height - 1 - j) as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            let output = format!("{} {} {}\n", ir, ig, ib);
            file.write(output.as_bytes()).unwrap();
        }
    }

    println!("\nDone!");
}
