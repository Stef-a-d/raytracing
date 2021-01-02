use crate::vec3::{Point3, Vec3};
use crate::ray::{Hittable, Ray, HitRecord};
use std::rc::Rc;
use crate::material::Material;
use crate::aabb::Aabb;
use std::f64::consts::PI;

pub struct Sphere{
    pub center: Point3, pub radius: f64, pub material: Rc<dyn Material>,
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
        let (u, v) = get_sphere_uv(&normal);
        Some(HitRecord{p, normal, t: root, u, v, front_face, material: self.material.clone()})

    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.center - Vec3::new(self.radius, self.radius, self.radius),
                       self.center + Vec3::new(self.radius, self.radius, self.radius)))
    }
}

fn get_sphere_uv(p: &Point3) -> (f64, f64) {
    let theta = (-p).y().acos();
    let phi = (-p).z().atan2(p.x()) + PI;
    let u = phi/(2.0*PI);
    let v = theta / PI;
    (u, v)
}

pub struct MovingSphere{
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn center(&self, time:f64) -> Point3 {
        self.center0 + (self.center1-self.center0)*((time - self.time0)/(self.time1 - self.time0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &ray.origin - &self.center(ray.time);
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
        let normal = (&p - &self.center(ray.time))/self.radius;
        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face {
            normal
        }else{
            -normal
        };
        let (u, v) = get_sphere_uv(&normal);
        Some(HitRecord{p, normal, t: root, u, v, front_face, material: self.material.clone()})
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb::new(self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
                             self.center(time0) + Vec3::new(self.radius, self.radius, self.radius));
        let box1 = Aabb::new(self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
                             self.center(time1) + Vec3::new(self.radius, self.radius, self.radius));
        Some(Aabb::surrounding_box(&box0, &box1))
    }
}