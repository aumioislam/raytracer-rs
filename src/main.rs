use std::io;

pub mod array3;
pub mod ray;
pub mod sphere;
pub mod camera;
pub mod interval;
pub mod util;

use crate::array3::*;
use crate::ray::*;
use crate::sphere::*;
use crate::camera::Camera;
use crate::util::*;

fn main() -> io::Result<()> {
    //world
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    world.push(Box::new(Sphere::new(Array3::new([0.0, 0.0, -1.0]), 0.5)));
    world.push(Box::new(Sphere::new(Array3::new([0.0, -100.5, -1.0]), 100.0)));

    // render
    let cam = Camera::new(16.0/9.0, 400, 100);
    cam.render(&world)
}
