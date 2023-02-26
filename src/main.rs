use std::{fs::File, io::Write};

use ray::Ray;
use vec_util::{dot, unit_vector};

use crate::{
    color_util::write_color,
    vec::{Color3, Point3, Vec3},
};

mod color_util;
mod ray;
mod vec;
mod vec_util;

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let origin_center = ray.get_origin().clone() - *center;
    let a = ray.get_direction().length_squared();
    let half_b = dot(&origin_center, ray.get_direction());
    let c = origin_center.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: &Ray) -> Color3 {
    let t = hit_sphere(&Point3::from(0.0, 0.0, -1.0), 0.5, ray);
    
    if (t > 0.0) {
        let vec = ray.at(t) - Vec3::from(0.0, 0.0, -1.0);
        let N = unit_vector(&vec);
        return Color3::from(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) * 0.5;
    }

    let unit_direction = unit_vector(ray.get_direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color3::from(1.0, 1.0, 1.0) * (1.0 - t) + Color3::from(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from(0.0, 0.0, 0.0);
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

    // Render
    let mut file = File::create("output.ppm").unwrap();
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    file.write(header.as_bytes()).unwrap();

    for j in 0..image_height {
        print!("\rScanlines remaining: {:04}", image_height - 1 - j);
        std::io::stdout().flush().unwrap();
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = (image_height - 1 - j) as f64 / (image_height - 1) as f64;

            let ray = Ray::from(
                origin.clone(),
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(&ray);

            write_color(&mut file, &pixel_color);
        }
    }

    println!("\nDone!");
}
