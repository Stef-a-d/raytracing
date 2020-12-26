use crate::ray::{HitRecord, Ray};
use crate::vec3::{Color, Vec3};

pub struct Scatter{
    pub attenuation: Color,
    pub scatter: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        let scatter = if scatter_direction.near_zero() {
            Ray::new(rec.p, rec.normal)
        }else {
            Ray::new(rec.p, scatter_direction)
        };
        let attenuation = self.albedo;
        Some(Scatter{
            attenuation, scatter,
        })
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian{albedo}
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = r_in.direction.unit().reflect(&rec.normal);
        let scatter = Ray::new(rec.p, reflected+ Vec3::random_in_unit_sphere() * self.fuzz);
        let attenuation = self.albedo;
        if scatter.direction.dot(&rec.normal) > 0.0 {
            Some(Scatter {
                scatter, attenuation,
            })
        }else {
            None
        }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal{
        if fuzz < 1.0 {
            Metal{albedo, fuzz}
        } else{
            Metal {albedo, fuzz: 1.0}
        }
    }
}