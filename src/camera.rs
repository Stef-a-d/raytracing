use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::random_double;

pub struct Camera{
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera{
    pub fn new(lookfrom:Point3, lookat:Point3, vup:Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64, time0: f64, time1: f64) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = (&vup.cross(&w)).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - (w*focus_dist);

        let lens_radius = aperture/2.0;
        Camera{
            origin, lower_left_corner, horizontal, vertical, u, v, w, lens_radius, time0, time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray{
        let rd =  Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.origin - offset,
            time: random_double(self.time0, self.time1),
        }
    }
}