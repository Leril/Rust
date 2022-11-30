use derive_more::Constructor;

use crate::{
    vector::{Point, Vec3, Color},
     v, 
     object::Object
};


#[derive(Debug, PartialEq, PartialOrd, Clone, Constructor)]
pub struct Ray{
    pub origin: Point,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t:f64) -> Point{
        self.origin + self.direction * t
    }
}

pub fn color(scene: &impl Object,ray: &Ray, depth: u8) -> Color{
    if depth == 0{
        return v!(0);
    }

    if let Some(hit) = scene.hit(ray, (0.00001, f64::INFINITY)) {
        if let Some(reflection) = hit.reflection{
            reflection.collour_attenuation * color(scene, &reflection.ray, depth - 1)
        }else {
            v!(0, 0, 0)
        }
    }else{
        let direction = ray.direction.normalise();
        let t = 0.5 * (direction.normalise().y + 1.0);

        let white: Color = v!(1);
        let blue: Color = v!(0.3, 0.5, 1);

        blue * t + white * (1.0 - t)
    }
}