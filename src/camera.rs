use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

pub struct Camera{
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera{
    pub fn new(lookfrom:Point3, lookat:Point3, vup:Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = (&vup.cross(&w)).unit();
        let v = &w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w;
        Camera{
            origin, lower_left_corner, horizontal, vertical
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray{
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.origin,
        }
    }
}