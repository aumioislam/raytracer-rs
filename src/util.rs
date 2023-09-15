use std::io::{self, Write, StdoutLock};

use crate::array3::*;
use crate::ray::*;

pub type Pixel = Array3;

pub fn write_pixel(os: &mut StdoutLock, pix: Pixel) -> io::Result<()> {
    let pix = 255.99 * pix;
    write!(os, "{} {} {}\n", pix[0].floor(), pix[1].floor(), pix[2].floor())?;
    Ok(())
}

pub fn ray_color(r: &Ray) -> Pixel {
    let unit_vec = r.direction.unit();
    let a = 0.5*(unit_vec[1] + 1.0);
    (1.0-a)*Pixel::new([1.0, 1.0, 1.0]) + a*Pixel::new([0.5, 0.7, 1.0])
}
