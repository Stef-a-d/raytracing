#![feature(iterator_fold_self)]
use crate::ray::{Hittable, Ray, HitRecord};
use std::rc::Rc;
use crate::aabb::Aabb;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>
}

impl HittableList{
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn new() -> HittableList {
        HittableList{
            objects: Vec::new()
        }
    }
}

impl Hittable for HittableList{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest = t_max;
        for object in &self.objects {
            let hit = object.hit(ray, t_min, closest);
            if hit.is_some(){
                hit_record = hit;
                closest = hit_record.as_ref().unwrap().t;
            }
        }
        hit_record
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None
        }
        let mut result = None;
        for object in &self.objects {
            let tmp_box = object.bounding_box(time0, time1);
            if tmp_box.is_none() {
                return None
            }
            match result {
                None => result = tmp_box,
                Some(r) => result = tmp_box.map(|tmp| Aabb::surrounding_box(&tmp, &r)),
            }
        }
        result
    }
}