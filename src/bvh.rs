use crate::ray::{Hittable, Ray, HitRecord};
use std::rc::Rc;
use crate::aabb::Aabb;
use std::mem::take;
use crate::random_int;
use std::cmp::Ordering;

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(src_objects: &mut Vec<Rc<dyn Hittable>>, time0: f64, time1: f64) -> BvhNode {
        let (left, right) = if src_objects.len() == 1 {
            let left = src_objects.first().unwrap().clone();
            let right = src_objects.first().unwrap().clone();
            (left, right)
        } else if src_objects.len() == 2 {
            let left = src_objects.get(0).unwrap().clone();
            let right = src_objects.get(1).unwrap().clone();
            (left, right)
        } else {
            let axis = random_int(0,2) as usize;
            src_objects.sort_by(|a, b| {
                let box_a = a.bounding_box(time0, time1).unwrap();
                let box_b = b.bounding_box(time0, time1).unwrap();
                box_a.min().e()[axis].partial_cmp(&box_b.min().e()[axis]).unwrap()
            });
            let mid = src_objects.len()/2;
            let left: Rc<dyn Hittable> = Rc::new(BvhNode::new(&mut src_objects[0..mid].to_vec(), time0, time1));
            let right: Rc<dyn Hittable> = Rc::new(BvhNode::new(&mut src_objects[mid..].to_vec(), time0, time1));
            (left,right)
        };
        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();
        let bbox = Aabb::surrounding_box(&box_left, &box_right);
        BvhNode {left, right, bbox }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bbox.hit(ray, t_min, t_max) {
            return None
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = self.right.hit(ray, t_min, t_max);

        hit_left.or(hit_right)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.bbox.min(), self.bbox.max()))
    }
}