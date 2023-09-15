use std::io::{self, Write, StdoutLock};

use crate::array3::*;
use crate::ray::*;

pub type Pixel = Array3;

pub fn write_pixel(os: &mut StdoutLock, pix: Pixel) -> io::Result<()> {
    let pix = 255.99 * pix;
    write!(os, "{} {} {}\n", pix[0].floor(), pix[1].floor(), pix[2].floor())?;
    Ok(())
}

pub fn hit_sphere(center: &Array3, radius: f64, r: &Ray) -> bool {
    let along_centers = r.origin - *center;
    
    let a = r.direction.square();
    let b = 2.0 * r.direction.dot(&along_centers);
    let c = along_centers.square() - radius*radius;

    let discrim = b*b - 4.0*a*c;

    discrim >= 0.0
}

pub fn ray_color(r: &Ray) -> Pixel {
    if hit_sphere(&Array3::new([0.0, 0.0, -1.0]), 0.5, r) {
        Pixel::new([1.0, 0.0, 0.0])
    } else {
        let unit_vec = r.direction.unit();
        let a = 0.5*(unit_vec[1] + 1.0);
        (1.0-a)*Pixel::new([1.0, 1.0, 1.0]) + a*Pixel::new([0.5, 0.7, 1.0])
    }
}
