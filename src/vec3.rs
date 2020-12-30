use core::ops;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e:[f64;3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{e:[x,y,z]}
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        (self.x() * rhs.x()) + (self.y() * rhs.y()) + (self.z() * rhs.z())
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            e: [(self.y() * rhs.z()) - (self.z() * rhs.y()),
            (self.z() * rhs.x()) - (self.x() * rhs.z()),
            (self.x() * rhs.y()) - (self.y() * rhs.x())],
        }
    }

    pub fn unit(&self) -> Vec3 {
        self / self.length()
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3::new(rand::thread_rng().gen_range(min..max), rand::thread_rng().gen_range(min..max), rand::thread_rng().gen_range(min..max))
    }

    pub fn random_in_unit_sphere() -> Vec3{
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - &((n * self.dot(n)) * 2.0)
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3{
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = (self + &(n * cos_theta)) * etai_over_etat;
        let r_out_parallel = n*(-((1.0 - r_out_perp.length_squared()).abs().sqrt()));
        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3 { e: [rand::thread_rng().gen_range(-1.0..1.0), rand::thread_rng().gen_range(-1.0..1.0), 0.0]};
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p
        }
    }

    pub fn x(&self) -> f64{
        self.e[0]
    }
    pub fn y(&self) -> f64{
        self.e[1]
    }
    pub fn z(&self) -> f64{
        self.e[2]
    }

    pub fn r(&self) -> f64{
        self.e[0]
    }
    pub fn g(&self) -> f64{
        self.e[1]
    }
    pub fn b(&self) -> f64{
        self.e[2]
    }

    pub fn e(&self) -> [f64;3] {
        self.e
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 { e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()] }
    }
}
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()] }
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 { e: [self.x() * rhs, self.y() * rhs, self.z() * rhs] }
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 { e: [self.x() * rhs, self.y() * rhs, self.z() * rhs] }
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 { e: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()] }
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()] }
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { e: [-self.x(), -self.y(), -self.z()] }
    }
}
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { e: [-self.x(), -self.y(), -self.z()] }
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        self + &(-rhs)
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;