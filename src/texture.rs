use crate::vec3::{Point3, Color, Vec3};
use std::rc::Rc;

pub trait Texture {
    fn value(&self, u:f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> SolidColor {
        SolidColor {
            color_value: c,
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Color, even: Color) -> CheckerTexture {
        CheckerTexture {
            odd: Rc::new(SolidColor::new(odd)),
            even: Rc::new(SolidColor::new(even)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0*p.x()).sin() * (10.0 * p.y()).sin() * (10.0*p.z()).sin();
        if (sines < 0.0){
            self.odd.value(u, v, p)
        }else{
            self.even.value(u, v, p)
        }
    }
}