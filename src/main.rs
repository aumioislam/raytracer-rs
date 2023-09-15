use std::io::{self, Write};

pub mod array3;
pub mod ray;
pub mod util;

use crate::array3::*;
use crate::ray::*;
use crate::util::*;

fn main() -> io::Result<()> {
    let mut cout = io::stdout().lock();
    let mut cerr = io::stderr().lock();

    // image
    let aspect_ratio = 16.0/9.0; //16:9 
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio).floor() as i32;
    let img_height = if img_height < 1 { 1 } else { img_height };

    //camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
    let camera = Array3::zero();

    // define viewport vectors
    let vp_u = Array3::new([viewport_width, 0.0,0.0]);
    let vp_v = Array3::new([0.0, -viewport_height, 0.0]);

    // define delta vectors
    let delta_u = vp_u / img_width as f64;
    let delta_v = vp_v / img_height as f64;
    
    let viewport_upper_left = camera - Array3::new([0.0, 0.0, focal_length]) - vp_u/2.0 - vp_v/2.0;
    let pix00_loc = viewport_upper_left + 0.5*(delta_u + delta_v);

    // render
    write!(&mut cout, "P3\n{} {}\n255\n", img_width, img_height)?;
    for j in 0..img_height {
        // write!(&mut cerr, "Scanlines remaining: {}\n", img_height-j)?;
        cerr.flush()?;
        for i in 0..img_width {
            let pixel_loc = pix00_loc + (delta_u * i as f64) + (delta_v * j as f64);
            let ray_dir = pixel_loc - camera;
            let r = Ray::new(camera, ray_dir);
            let pix = ray_color(&r);

            write_pixel(&mut cout, pix)?;
        }
    }

    write!(&mut cerr, "Done\n")?;
    cerr.flush()?;
    Ok(())
}
