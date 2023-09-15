use std::io::{self, Write, StdoutLock};
use rand::Rng;

use crate::array3::*;
use crate::interval::*;
use crate::camera::Pixel;

pub fn write_pixel(os: &mut StdoutLock, pix: Pixel, samples: i32) -> io::Result<()> {
    let pix = pix / samples as f64;

    let intensity = Interval::new(0.0, 0.999);

    let r = 256.0 * intensity.clamp(pix[0]);
    let g = 256.0 * intensity.clamp(pix[1]);
    let b = 256.0 * intensity.clamp(pix[2]);

    write!(os, "{} {} {}\n", r.floor(), g.floor(), b.floor())?;
    Ok(())
}

pub fn random_f64() -> f64 {
   rand::thread_rng().gen_range(0.0..1.0)
}
