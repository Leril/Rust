use derive_more::Constructor;

use crate::{ray::Ray, vector::{Color, Vec3}, object::Hit, v};



#[derive(Debug)]
pub struct Reflection{
    pub ray: Ray,
    pub collour_attenuation: Color,
}

pub trait Material{
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection>;
}

#[derive(Debug, Constructor)]
pub struct Lambertian(Color);

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<Reflection> {
        let mut scattered_direction = hit.normal + Vec3::random_unit();
        
        if scattered_direction.is_zero(){
            scattered_direction = hit.normal;
        }
        
        let reflected_ray = Ray::new(hit.impact_point, scattered_direction);

        Some(Reflection { ray: reflected_ray, collour_attenuation: self.0 })
    }
}


#[derive(Debug, Constructor)]
pub struct Metal{
    color: Color, 
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let reflection = reflect(incident_ray.direction, &hit.normal) + self.fuzz * Vec3::random_unit();

        let scattered = Ray::new(hit.impact_point, reflection);

        if scattered.direction.dot(&hit.normal) > 0.0{
            Some(Reflection { ray: scattered, collour_attenuation: self.color})
        }else{
            None
        }
    }
}

#[derive(Debug, Constructor)]
pub struct Dielectric(f64);

impl Material for Dielectric {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let ratio = if hit.front_face {1.0 / self.0} else {self.0};

        let unit_direction = incident_ray.direction.normalise();

        let cos_theta = -unit_direction.dot(&hit.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let scatter_direction = if (ratio * sin_theta) > 1.0 || reflectance(cos_theta, ratio) > rand::random(){
            reflect(unit_direction, &hit.normal)
        } else {
            refract(unit_direction, &hit.normal, ratio)
        };

        let out_ray = Ray::new(hit.impact_point, scatter_direction);

        Some(Reflection{
            ray: out_ray,
            collour_attenuation: v!(1),
        })
    }
}

fn reflect(v: Vec3, normal: &Vec3) -> Vec3{
    v - 2.0 * v.dot(normal) * *normal
}

fn refract(incident: Vec3, normal: &Vec3, ratio: f64) -> Vec3{
    let cos = -incident.dot(normal);
    let r_out_prep = ratio * (incident + cos * *normal);
    let r_par_out = -0.1 * ( - r_out_prep.dot(&r_out_prep).abs().sqrt() * *normal);
    r_out_prep + r_par_out
}

fn reflectance(cos: f64, n: f64) -> f64{
    let r0 = f64::powi((1.0 - n)/(1.0 + n), 2);
    r0 + (1.0 + r0) * f64::powi(1.0 - cos, 5)
}
