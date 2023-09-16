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
use crate::material::Dielectric;
use crate::util::*;

fn main() -> io::Result<()> {
    //world
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    //ground
    let mat_ground  = Material::Lambertian(Lambertian { albedo: Array3::new([0.5, 0.5, 0.5]) });
    world.push(Box::new(Sphere::new(Array3::new([0.0, -1000.0, 0.0]), 1000.0, &mat_ground)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Array3::new([a as f64 + 0.9*random_f64(), 0.2, b as f64 + 0.9*random_f64()]);

            if (center - Array3::new([4.0, 0.2, 0.0])).norm() > 0.9 {
                let mat_sphere = if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Array3::random() * Array3::random();
                    Material::Lambertian(Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    //metal 
                    let albedo = Array3::random_rge(0.5, 1.0);
                    let fuzz = random_rge_f64(0.0, 0.5);
                    Material::Metal(Metal { albedo, fuzz })
                } else {
                    //glass
                    Material::Dielectric(Dielectric { index_r: 1.5 })
                };
                world.push(Box::new(Sphere::new(center, 0.2, &mat_sphere)));
            }
        }
    }

    let mat1 = Material::Dielectric(Dielectric { index_r: 1.5 });
    world.push(Box::new(Sphere::new(Array3::new([0.0, 1.0, 0.0]), 1.0, &mat1)));

    let mat2 = Material::Lambertian(Lambertian { albedo: Array3::new([0.4, 0.2, 0.1]) });
    world.push(Box::new(Sphere::new(Array3::new([-4.0, 1.0, 0.0]), 1.0, &mat2)));

    let mat3 = Material::Metal(Metal { albedo: Array3::new([0.7, 0.6, 0.5]), fuzz: 0.0 });
    world.push(Box::new(Sphere::new(Array3::new([4.0, 1.0, 0.0]), 1.0, &mat3)));

    // render
    let aspect_ratio = 16.0/9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov = 20.0;
    let lookfrom = Array3::new([13.0, 6.0, 3.0]);
    let lookat = Array3::new([0.0, 0.0, 0.0]);
    let vup = Array3::new([0.0, 1.0, 0.0]);

    let defocus_angle   = 0.6;
    let focus_dist      = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist
    );
    cam.render(&world)
}
