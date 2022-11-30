use derive_more::Constructor;

use crate::{ray::Ray, vector::{Point, Vec3}, material::{Reflection, Material}};

pub struct Hit{
    pub impact_point: Point,
    pub normal: Vec3,
    pub parameter: f64,
    pub front_face: bool,
    pub reflection: Option<Reflection>, 
}

pub trait Object {
    
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) ->Option<Hit>;
}

#[derive(Constructor)]
pub struct Sphere<M: Material>{
    center: Point,
    radius: f64,
    mateial: M,
}

impl <M: Material> Object for Sphere<M> {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let d = b * b - 4.0 * a * c;

        if d < 0.0{
            return None;
        }

        let mut root = (-b - d.sqrt()) / (2.0 * a);

        if !(bounds.0..bounds.1).contains(&root){
            root = (-b + d.sqrt()) / (2.0 * a);
            if !(bounds.0..bounds.1).contains(&root){
                return None;
            }
        }

        let impact_point = ray.at(root);
        let normal = (impact_point - self.center) / self.radius;

        let (normal, front_face) = if ray.direction.dot(&normal) > 0.0{
            (-normal, false)
        }else{
            (normal, true)
        };

        let mut hit = Hit{
            impact_point,
            normal,
            parameter: root,
            front_face,
            reflection: None,
        };

        hit.reflection = self.mateial.scatter(ray, &hit);
        Some(hit)
    }
}

pub type Scene = Vec<Box<dyn Object + Sync>>;

impl Object for Scene {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) ->Option<crate::object::Hit> {
        self.iter()
            .filter_map(|o| o.hit(ray, bounds))
            .min_by(|h1, h2| h1.parameter.partial_cmp(&h2.parameter).unwrap())
    }
}