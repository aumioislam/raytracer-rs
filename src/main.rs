use std::io;

pub mod array3;
pub mod ray;
pub mod sphere;
pub mod camera;
pub mod material;
pub mod interval;
pub mod util;

use crate::array3::*;
use crate::ray::*;
use crate::sphere::*;
use crate::camera::Camera;
use crate::material::Material;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::util::*;

fn main() -> io::Result<()> {
    // materials
    let mat_ground  = Material::Lambertian(Lambertian { albedo: Array3::new([0.8, 0.8, 0.0]) });
    let mat_center  = Material::Lambertian(Lambertian { albedo: Array3::new([0.7, 0.3, 0.3]) });
    let mat_left    = Material::Metal(Metal { albedo: Array3::new([0.8, 0.8, 0.8]), fuzz: 0.3 }); 
    let mat_right   = Material::Metal(Metal { albedo: Array3::new([0.8, 0.6, 0.2]), fuzz: 1.0 });

    //world
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    world.push(Box::new(Sphere::new(Array3::new([0.0, -100.5, -1.0]), 100.0, mat_ground)));
    world.push(Box::new(Sphere::new(Array3::new([0.0, 0.0, -1.0]), 0.5, mat_center)));
    world.push(Box::new(Sphere::new(Array3::new([-1.0, 0.0, -1.0]), 0.5, mat_left)));
    world.push(Box::new(Sphere::new(Array3::new([1.0, 0.0, -1.0]), 0.5, mat_right)));

    // render
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let cam = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);
    cam.render(&world)
}
