use crate::vec3::{Point3, Vec3};
use std::rc::Rc;
use crate::material::Material;
use crate::aabb::Aabb;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Ray {
        Ray{origin, direction, time}
    }
}

pub struct HitRecord {
    pub p: Point3, pub normal: Vec3, pub t: f64, pub u: f64, pub v: f64, pub front_face: bool, pub material: Rc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}