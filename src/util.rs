use std::io::{self, Write, StdoutLock};

use crate::array3::*;
use crate::interval::*;

pub type Pixel = Array3;

pub fn write_pixel(os: &mut StdoutLock, pix: Pixel) -> io::Result<()> {
    let pix = 255.99 * pix;
    write!(os, "{} {} {}\n", pix[0].floor(), pix[1].floor(), pix[2].floor())?;
    Ok(())
}
