mod geometry;
mod ray;
mod sphere;
mod hittable_list;
mod camera;

use crate::geometry::{Vec3, Color, Point3};
use crate::ray::{Ray, Hittable};
use crate::hittable_list::HittableList;
use std::rc::Rc;
use crate::sphere::Sphere;
use crate::camera::Camera;
use rand::Rng;

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 100;

    // World

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere{center: Point3::new(0.0, 0.0, -1.0), radius: 0.5}));
    world.add(Rc::new(Sphere{center: Point3::new(0.0, -100.5, -1.0), radius: 100.0}));


    // Camera
    let camera = Camera::new();

    // Render

    println!("P3");
    println!("{0} {1}", image_width, image_height);
    println!("255");
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining {0}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel{
                let u = (i as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (image_width as f64 - 1.0);
                let v = (j as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world);
            }
            write_color(&pixel_color, samples_per_pixel);
        }
    }

    eprintln!("Done");
}

fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x < min{
        min
    } else if x > max{
        max
    } else{
        x
    }
}

fn write_color(clr: &Color, samples_per_pixel: i32) {
    let scale = 1.0 / (samples_per_pixel as f64);
    let r = clr.x * scale;
    let g = clr.y * scale;
    let b = clr.z * scale;

    let ir: i32 = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let ig: i32 = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let ib: i32 = (256.0 * clamp(b, 0.0, 0.999)) as i32;

    println!("{0} {1} {2}", ir, ig, ib);
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let record = world.hit(ray, 0.0, f64::MAX);
    record.map_or_else(
        || {
            let unit_direction: Vec3 = ray.direction.unit();
            let t: f64 = 0.5 * (unit_direction.y + 1.0);
            (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
        },
        |rec|
            (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5)
}