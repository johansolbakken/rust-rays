use std::{f64::INFINITY, fs::File, io::Write};

use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec::Vec3;
use vec_util::unit_vector;

use crate::{
    camera::Camera,
    color_util::write_color,
    hittable_list::HittableList,
    math::random_double,
    sphere::Sphere,
    vec::{Color3, Point3},
};

mod camera;
mod color_util;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod sphere;
mod vec;
mod vec_util;

fn ray_color(ray: &Ray, world: &Box<dyn Hittable>, depth: u32) -> Color3 {
    if depth <= 0 {
        return Color3::new();
    }

    let mut rec = HitRecord::new();
    if world.hit(ray, 0.001, INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_unit_vector();
        return ray_color(&Ray::from(rec.p, target - rec.p), world, depth - 1) * 0.5;
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
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;

    // World
    let mut world = HittableList::new();
    world.add(Sphere::box_from(Point3::from(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::box_from(Point3::from(0.0, -100.5, -1.0), 100.0));

    let world_ref: Box<dyn Hittable> = Box::from(world);

    // Camera
    let camera = Camera::new(aspect_ratio);

    // Render
    let mut file = File::create("output.ppm").unwrap();
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    file.write(header.as_bytes()).unwrap();

    for j in 0..image_height {
        print!("\rScanlines remaining: {:04}", image_height - 1 - j);
        std::io::stdout().flush().unwrap();

        for i in 0..image_width {
            let mut pixel_color = Color3::new();

            for s in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (image_height as f64 - 1.0 - j as f64 + random_double())
                    / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world_ref, max_depth);
            }

            write_color(&mut file, &pixel_color, samples_per_pixel);
        }
    }

    println!("\nDone!");
}
