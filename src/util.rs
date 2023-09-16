use std::io::{self, Write, StdoutLock};
use rand::Rng;

use crate::array3::Array3;
use crate::interval::Interval;
use crate::camera::Pixel;

pub fn write_pixel(os: &mut StdoutLock, pix: Pixel, samples: i32) -> io::Result<()> {
    let pix = pix / samples as f64;

    let intensity = Interval::new(0.0, 0.999);

    let r = lin_to_gamma(pix[0]);
    let g = lin_to_gamma(pix[1]);
    let b = lin_to_gamma(pix[2]);

    let r = 256.0 * intensity.clamp(r);
    let g = 256.0 * intensity.clamp(g);
    let b = 256.0 * intensity.clamp(b);

    write!(os, "{} {} {}\n", r.floor(), g.floor(), b.floor())?;
    Ok(())
}

fn lin_to_gamma(x: f64) -> f64 {
    x.sqrt()
}

pub fn random_f64() -> f64 {
   rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_rge_f64(min: f64, max: f64) -> f64 {
   rand::thread_rng().gen_range(min..max)
}

pub fn degrees_to_rad(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI/180.0
}
