use std::{f64::INFINITY, fs::File, io::Write, rc::Rc};

use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec::Vec3;
use vec_util::unit_vector;

use crate::{
    camera::Camera,
    color_util::write_color,
    hittable_list::HittableList,
    material::{Lambertian, Metal},
    math::random_double,
    sphere::{Sphere, SphereBuilder},
    vec::{Color3, Point3},
};

mod camera;
mod color_util;
mod hittable;
mod hittable_list;
mod material;
mod math;
mod ray;
mod sphere;
mod vec;
mod vec_util;

fn ray_color(ray: &Ray, world: &Rc<dyn Hittable>, depth: u32) -> Color3 {
    if depth <= 0 {
        return Color3::new();
    }

    let mut rec = HitRecord::new();
    if world.hit(ray, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::new();
        let mut attenuation = Color3::new();
        if rec
            .material
            .scatter(ray, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color3::new();
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
    let material_ground = Rc::new(Lambertian {
        albedo: Color3::from(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color3::from(0.7, 0.3, 0.3),
    });
    let material_left = Rc::new(Metal {
        albedo: Color3::from(0.8, 0.8, 0.8),
    });
    let material_right = Rc::new(Metal {
        albedo: Color3::from(0.8, 0.6, 0.2),
    });

    let mut world = HittableList::new();
    world.add(
        SphereBuilder::new()
            .set_position(Point3::from(0.0, -100.5, -0.1))
            .set_radius(100.0)
            .set_material(material_ground)
            .to_rc(),
    );
    world.add(
        SphereBuilder::new()
            .set_position(Point3::from(0.0, 0.0, -1.0))
            .set_radius(0.5)
            .set_material(material_center)
            .to_rc(),
    );
    world.add(
        SphereBuilder::new()
            .set_position(Point3::from(-1.0, 0.0, -1.0))
            .set_radius(0.5)
            .set_material(material_left)
            .to_rc(),
    );
    world.add(
        SphereBuilder::new()
            .set_position(Point3::from(1.0, 0.0, -1.0))
            .set_radius(0.5)
            .set_material(material_right)
            .to_rc(),
    );

    let world_ref: Rc<dyn Hittable> = Rc::from(world);

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
