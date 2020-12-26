use crate::vec3::{Point3, Vec3};
use std::rc::Rc;
use crate::material::Material;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray{origin, direction}
    }
}

pub struct HitRecord {
    pub p: Point3, pub normal: Vec3, pub t: f64, pub front_face: bool, pub material: Rc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}