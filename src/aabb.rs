use crate::vec3::Point3;
use crate::ray::Ray;

pub struct Aabb {
    minimum: Point3,
    maximum: Point3,
}

impl Aabb {
    pub fn new(min: Point3, max: Point3) -> Aabb {
        Aabb {minimum: min, maximum: max}
    }

    pub fn min(&self) -> Point3 {
        self.minimum
    }
    pub fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn naive_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let t0 = ((self.minimum.e()[a] - ray.origin.e()[a])/ray.direction.e()[a]).min(
                (self.maximum.e()[a] - ray.origin.e()[a])/ray.direction.e()[a]
            );
            let t1 = ((self.minimum.e()[a] - ray.origin.e()[a])/ray.direction.e()[a]).max(
                (self.maximum.e()[a] - ray.origin.e()[a])/ray.direction.e()[a]
            );
            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);
            if t_max <= t_min {
                return false
            }
        }
        return true
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let invD = 1.0/ray.direction.e()[a];
            let mut t0 = (self.min().e()[a] - ray.origin.e()[a])*invD;
            let mut t1 = (self.max().e()[a] - ray.origin.e()[a])*invD;
            if invD < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false
            }
        }
        true
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        let small = Point3::new(box0.min().x().min(box1.min().x()),
                                box0.min().y().min(box1.min().y()),
                                box0.min().z().min(box1.min().z()));
        let big = Point3::new(box0.max().x().max(box1.max().x()),
                                box0.max().y().max(box1.max().y()),
                                box0.max().z().max(box1.max().z()));
        Aabb {minimum: small, maximum: big,}

    }
}
