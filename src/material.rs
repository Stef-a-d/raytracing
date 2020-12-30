use crate::ray::{HitRecord, Ray};
use crate::vec3::{Color, Vec3};
use rand::Rng;

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
            Ray::new(rec.p, rec.normal, r_in.time)
        }else {
            Ray::new(rec.p, scatter_direction, r_in.time)
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
        let scatter = Ray::new(rec.p, reflected+ Vec3::random_in_unit_sphere() * self.fuzz, r_in.time);
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

pub struct Dielectric{
    ir: f64
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0/self.ir
        }else{
            self.ir
        };
        let unit_direction = r_in.direction.unit();

        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0-cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction  = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen_range(0.0..1.0) {
            unit_direction.reflect(&rec.normal)
        }else{
            unit_direction.refract(&rec.normal, refraction_ratio)
        };
        let scatter = Ray::new(rec.p, direction, r_in.time);
        Some(Scatter {attenuation, scatter})
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric{
        Dielectric{ir}
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0-ref_idx) / (1.0+ref_idx)).powi(2);
        r0 + (1.0-r0) * (1.0-cosine).powi(5)
    }
}