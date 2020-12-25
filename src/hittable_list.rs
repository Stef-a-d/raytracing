use crate::ray::{Hittable, Ray, HitRecord};
use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>
}

impl HittableList{
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
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
}