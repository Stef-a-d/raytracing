mod geometry;
mod ray;
mod sphere;
mod hittable_list;

use crate::geometry::{Vec3, Color, Point3};
use crate::ray::Ray;

fn main() {

    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3");
    println!("{0} {1}", image_width, image_height);
    println!("255");
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining {0}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let r = Ray {origin, direction: lower_left_corner + (horizontal * u) + (vertical * v) - origin};
            let pixel_color = ray_color(&r);
            write_color(&pixel_color);
        }
    }

    eprintln!("Done");
}

fn write_color(clr: &Color) {
    let ir: i32 = (255.999 * clr.x) as i32;
    let ig: i32 = (255.999 * clr.y) as i32;
    let ib: i32 = (255.999 * clr.z) as i32;

    println!("{0} {1} {2}", ir, ig, ib);
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
        Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5
    } else {
        let unit_direction: Vec3 = ray.direction.unit();
        let t: f64 = 0.5*(unit_direction.y + 1.0);
        (Color::new(1.0, 1.0, 1.0) * (1.0-t)) + (Color::new(0.5, 0.7, 1.0) * t)
    }

}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = &ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let half_b = oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let disc = half_b * half_b - a * c;
    // disc > 0.0
    if disc < 0.0 {
        -1.0
    }else {
        (-half_b - disc.sqrt())/(a)
    }
}