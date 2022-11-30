use std::ops::Mul;

use derive_more::{Add, Sub, Mul, Div, Neg, Constructor};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Add, Sub, Mul, Div, Neg, Constructor)]
pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[macro_export]
macro_rules! v {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3::new(f64::from($x), f64::from($y), f64::from($z))
    };

    ($x:expr) => {
        Vec3::new(f64::from($x), f64::from($x), f64::from($x))
    };
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3{
    pub fn len(&self) -> f64{
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalise(self) -> Vec3{
        self / self.len()
    }

    pub fn dot(&self, other: &Self) -> f64{
        self.x * other.x + self.y * other.y + self.z *other.z
    }

    pub fn cross(&self, other: &Self) -> Self{
        Self {
             x: self.y * other.z - self.z * other.y,
             y: self.z * other.x - self.x * other.z,
             z: self.x * other.y - self.y * other.x
        }
    }

    pub fn to_rgb(self) -> image::Rgb<u8>{
        let values = [self.x, self.y, self.z]
                                .map(|c| c.sqrt())
                                .map(|c| (c * 255.999) as u8);

        image::Rgb(values)
    }

    pub fn map<F>(self, mut f: F) -> Vec3 where F: FnMut(f64) -> f64{
        Vec3 { x: f(self.x), y: f(self.y), z: f(self.z) }
    }

    pub fn is_zero(&self) -> bool{
        let tolerance: f64 = 1e-8;

        self.x.abs() < tolerance && self.y.abs() < tolerance && self.z.abs() < tolerance
    }

    pub fn random_unit() -> Self {
        loop{
            let v = v!(rand::random::<f64>() * 2.0 - 1.0);

            if v.len() < 1.0{
                break v.normalise();
            }
        }
    }


}

impl Mul<Vec3> for f64{
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.map(|x| x * self)
        
    }
} 

impl Mul for Vec3{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}