use crate::vec3::Point3;
use crate::ray::{Hittable, Ray, HitRecord};

pub struct Sphere{
    pub center: Point3, pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &ray.origin - &self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            return None
        }
        let sqrtd = disc.sqrt();

        let root = (-half_b - sqrtd)/a;
        if root < t_min || t_max < root {
            return None
        }
        let p = ray.at(root);
        let normal = (&p - &self.center)/self.radius;
        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face {
            normal
        }else{
            -normal
        };
        Some(HitRecord{p, normal, t: root, front_face})

    }
}