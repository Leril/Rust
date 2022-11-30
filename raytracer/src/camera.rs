use rand::prelude::Distribution;

use crate::{vector::{Vec3, Point}, ray::Ray, v};

pub const WIDTH: u32 = 1200;
pub const RATIO: f64 = 3_f64/2_f64;
pub const HEIGHT: u32 = ((WIDTH as f64) / RATIO) as u32;

pub struct Camera{
    origin: Point,
    top_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new( look_from: Vec3, look_at: Vec3, vup: Vec3, fov: f64, aspect_ratio: f64, aperture: f64, focus_distance: f64) -> Self {

        let angle = fov.to_radians();
        let h = f64::tan(angle / 2.0);
        let view_height = 2.0 * h;
        let view_width = view_height * aspect_ratio;
        
        let w = (look_from - look_at).normalise();
        let u = vup.cross(&w).normalise();
        let v = w.cross(&u);


        let origin = look_from;
        let horizontal = view_width * u * focus_distance;
        let vertical = view_height * v * focus_distance;
        
        let top_left = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

        let lens_radius = aperture / 2.0;
        Camera{
            origin,
            top_left,
            horizontal,
            vertical,
            u,
            v,
            lens_radius
        }
    }


    pub fn get_ray(&self, s:f64, t:f64) -> Ray{
        let rand = random_in_unit_circle() * self.lens_radius;
        let origin = self.origin + self.u * rand.x + self.v * rand.y;

        let px_position = self.top_left  + s * self.horizontal + t * self.vertical;
        Ray::new(origin, px_position - origin)
    }
}

fn random_in_unit_circle() -> Vec3{
    let dist = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);
    let mut rng = rand::thread_rng();

    loop{
        let v = v!(dist.sample(&mut rng), dist.sample(&mut rng), 0);

        if v.len() < 1.0{
            break v.normalise();
        }
    }
}
