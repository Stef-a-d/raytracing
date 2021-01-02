mod vec3;
mod ray;
mod sphere;
mod hittable_list;
mod camera;
mod material;
mod aabb;
mod bvh;
mod texture;

use crate::vec3::{Vec3, Color, Point3};
use crate::ray::{Ray, Hittable};
use crate::hittable_list::HittableList;
use std::rc::Rc;
use crate::sphere::{Sphere, MovingSphere};
use crate::camera::Camera;
use rand::Rng;
use crate::material::{Lambertian, Metal, Dielectric};
use crate::texture::{CheckerTexture, SolidColor, Texture};

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 100;
    let max_depth = 50;


    // World

    // let mut world = HittableList::new();
    //
    // let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let material_left = Rc::new(Dielectric::new(1.5));
    // let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    //
    // world.add(Rc::new(Sphere{center: Point3::new(0.0, -100.5, -1.0), radius: 100.0, material: material_ground.clone()}));
    // world.add(Rc::new(Sphere{center: Point3::new(0.0, 0.0, -1.0), radius: 0.5, material: material_center.clone()}));
    // world.add(Rc::new(Sphere{center: Point3::new(-1.0, 0.0, -1.0), radius: 0.5, material: material_left.clone()}));
    // world.add(Rc::new(Sphere{center: Point3::new(-1.0, 0.0, -1.0), radius: -0.45, material: material_left.clone()}));
    // world.add(Rc::new(Sphere{center: Point3::new(1.0, 0.0, -1.0), radius: 0.5, material: material_right.clone()}));

    let (world, lookfrom, lookat, vfov, aperture) = match 1 {
        0 => {
            let world = random_scene();
            let lookfrom = Point3::new(13.0,2.0, 3.0);
            let lookat = Point3::new(0.0, 0.0, 0.0);
            let aperture = 0.1;
            let vfov = 20.0;
            (world, lookfrom, lookat, vfov, aperture)
        },
        1 => {
            let world = two_spheres();
            let lookfrom = Point3::new(13.0,2.0, 3.0);
            let lookat = Point3::new(0.0, 0.0, 0.0);
            let aperture = 0.0;
            let vfov = 20.0;
            (world, lookfrom, lookat, vfov, aperture)
        }
        _ => {
            let world = random_scene();
            let lookfrom = Point3::new(13.0,2.0, 3.0);
            let lookat = Point3::new(0.0, 0.0, 0.0);
            let aperture = 0.1;
            let vfov = 20.0;
            (world, lookfrom, lookat, vfov, aperture)
        }
    };



    // Camera


    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        /*90.0*/vfov, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

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
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
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

    // Divide color by number of samples & gamma correct for gamma = 2
    let scale = 1.0 / (samples_per_pixel as f64);
    let r = (clr.x() * scale).sqrt();
    let g = (clr.y() * scale).sqrt();
    let b = (clr.z() * scale).sqrt();

    let ir: i32 = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let ig: i32 = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let ib: i32 = (256.0 * clamp(b, 0.0, 0.999)) as i32;

    println!("{0} {1} {2}", ir, ig, ib);
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0{
        return Color::new(0.0, 0.0, 0.0)
    }

    let record = world.hit(ray, 0.001, f64::MAX);
    record.map_or_else(
        || {
            let unit_direction: Vec3 = ray.direction.unit();
            let t: f64 = 0.5 * (unit_direction.y() + 1.0);
            (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
        },
        |rec| {
            let scatter = rec.material.scatter(ray, &rec);
            scatter.map_or_else(
                || {Color::new(0.0, 0.0, 0.0)},
                |scatter| {
                scatter.attenuation * ray_color(&scatter.scatter, world, depth - 1)
            })
        })
}

fn random_double(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

fn random_int(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Box::new(CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    let ground_material = Rc::new(Lambertian::new(checker));
    world.add(Rc::new(Sphere{center: Point3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: ground_material.clone()}));

    for a in -11..11 {
        for b in -11..11{
            let choose_mat = random_double(0.0, 1.0);
            let center = Point3::new(a as f64 + 0.9*random_double(0.0, 1.0), 0.2, b as f64 + 0.9*random_double(0.0, 1.0));

            if (center-Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere: Rc<dyn Hittable> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0, 1.0);
                    let material = Rc::new(Lambertian::new(Box::new(SolidColor::new(albedo))));
                    let center1 = center + Vec3::new(0.0, random_double(0.0, 0.5), 0.0);
                    Rc::new(MovingSphere {center0: center, center1: center1, time0: 0.0, time1: 1.0, radius: 0.2, material: material.clone()})
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.0, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    Rc::new(Sphere {center: center, radius: 0.2, material: material.clone()})
                } else {
                    // glass
                    let material = Rc::new(Dielectric::new(1.5));
                    Rc::new(Sphere {center: center, radius: 0.2, material: material.clone()})
                };
                world.add(sphere.clone());
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere {center: Point3::new(0.0, 1.0, 0.0), radius: 1.0, material: material1.clone()}));

    let material2 = Rc::new(Lambertian::new(Box::new(SolidColor::new(Color::new(0.4, 0.2, 0.1)))));
    world.add(Rc::new(Sphere {center: Point3::new(-4.0, 1.0, 0.0), radius: 1.0, material: material2.clone()}));

    let material3 = Rc::new(Metal::new(Color::new(0.7,0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere {center: Point3::new(4.0, 1.0, 0.0), radius: 1.0, material: material3.clone()}));

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker: Box<Texture> = Box::new(CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    let lambertian = Rc::new(Lambertian::new(checker));
    objects.add(Rc::new(Sphere {center: Point3::new(0.0, -10.0, 0.0), radius: 10.0, material: lambertian.clone()}));
    objects.add(Rc::new(Sphere {center: Point3::new(0.0, 10.0, 0.0), radius: 10.0, material: lambertian.clone()}));

    objects
}