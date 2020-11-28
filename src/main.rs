use core::ops;

fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    println!("P3");
    println!("{0} {1}", image_width, image_height);
    println!("255");
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining {0}", j);
        for i in 0..image_width {
            let clr = Color {
                x: f64::from(i) / f64::from(image_width - 1),
                y: f64::from(j) / f64::from(image_height - 1),
                z: 0.25,
            };
            write_color(&clr);
        }
    }

    eprintln!("Done");
}

fn write_color(clr: &Color) {
    let ir: i32 = (255.999 * clr.x) as i32;
    let ig: i32 = (255.999 * clr.y) as i32;
    let ib: i32 = (255.999 * clr.z) as i32;

    println!("{0} {1} {2}", ir, ig, ib);
}

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * self.y - self.y * self.x,
        }
    }

    fn unit(&self) -> Vec3 {
        self / self.length()
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        self + &(-rhs)
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

type Point3 = Vec3;
type Color = Vec3;

struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn at(self, t: f64) -> Point3 {
        &self.origin + &(&self.direction * t)
    }
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction: Vec3 = ray.direction.unit();
    let t: f64 = 0.5*(unit_direction.y + 1.0);
    &(&Color{ x: 0.0, y: 0.0, z: 0.0 } * (1.0-t)) + &(&Color{x: 0.5, y: 0.7, z: 1.0} * t)
}