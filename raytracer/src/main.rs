use std::path::Path;

use camera::{HEIGHT, WIDTH};
use image::{RgbImage};
use indicatif::{ProgressBar, ProgressStyle, ProgressFinish, ParallelProgressIterator};
use object::{Scene, Sphere};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use vector::Vec3;

use crate::{material::{Lambertian, Metal, Dielectric}, camera::RATIO};

mod vector;
mod ray;
mod object;
mod camera;
mod material;


fn main() {
    let samples: u32 = 100;
    let max_depth = 50;

    let look_from = v!(13, 2, 3);
    let look_at = v!(0, 0, 0);
    let camera = camera::Camera::new(
        look_from,
        look_at,
        v!(0, -1, 0),
        20.0,
        RATIO,
        0.1,
        10.0,
    );

    println!("Rendering the new scene...");
    let bar = ProgressBar::new((WIDTH * HEIGHT) as u64);
    bar.set_style(
        ProgressStyle::default_bar()
        .template(
            "{spinner:.green} [{wide_bar:.green/white}] {percent}% - {elapsed_precise} elapsed {msg}",
        ).progress_chars("#>-")
        .on_finish(ProgressFinish::WithMessage("Done ! enjoy".into())));

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    let objects: Scene = random_scene();

    img
        .enumerate_pixels_mut()
        .par_bridge()
        .progress_with(bar)
        .for_each(|pixel| {
            let mut color = v!(0);

            for _ in 0..samples{

                let u = (pixel.0 as f64 + rand::random::<f64>()) / (WIDTH - 1) as f64;
                let v = (pixel.1 as f64 + rand::random::<f64>()) / (HEIGHT - 1) as f64;
                
                let ray_direction = camera.get_ray(u, v);
                color = color + ray::color(&objects, &ray_direction, max_depth);
            }

            *pixel.2 = (color / samples as f64).to_rgb();
    
        });

    let target_path = Path::new("result.jpg");
    img.save(target_path).expect("Could not save image");
}

// fn assign_colors(rgb: &mut Rgb<u8>,r: f64, g: f64, b: f64){
//     rgb.0[0] = (r * 255.0) as u8;
//     rgb.0[1] = (g * 255.0) as u8;
//     rgb.0[2] = (b * 255.0) as u8;
// }

fn random_scene() -> Scene {
    let mut objects: Vec<Box<dyn object::Object + Sync>> = vec![];

    let ground = Box::new(Sphere::new(
        v!(0, -1000, 0),
        1000.0,
        Lambertian::new(v!(0.5, 0.5, 0.5)),
    ));
    objects.push(ground);

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let material_choice: f64 = rand::random();
            let center = v!(
                a + 0.9 * rand::random::<f64>(),
                0.2,
                b + 0.9 * rand::random::<f64>()
            );

            if material_choice < 0.8 {
                //diffuse
                let material = Lambertian::new(v!(rand::random::<f64>()));
                objects.push(Box::new(Sphere::new(center, 0.2, material)));
            } else if material_choice < 0.95 {
                //metal
                let colour = v!(rand::random::<f64>() / 2.0 + 0.5);
                let fuzz = rand::random::<f64>() / 2.0;
                let material = Metal::new(colour, fuzz);
                objects.push(Box::new(Sphere::new(center, 0.2, material)));
            } else {
                //glass
                objects.push(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
            }
        }
    }

    objects.push(Box::new(Sphere::new(
        v!(0, 1, 0),
        1.0,
        Dielectric::new(1.5),
    )));
    objects.push(Box::new(Sphere::new(
        v!(-4, 1, 0),
        1.0,
        Lambertian::new(v!(0.4, 0.2, 0.1)),
    )));
    objects.push(Box::new(Sphere::new(
        v!(4, 1, 0),
        1.0,
        Metal::new(v!(0.7, 0.6, 0.5), 0.0),
    )));
    objects
}
