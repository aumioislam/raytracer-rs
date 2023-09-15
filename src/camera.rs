use std::io::{self, Write};

use crate::array3::Array3;
use crate::ray::*;
use crate::interval::*;
use crate::util::*;

pub struct Camera {
    aspect_ratio: f64,
    img_width: i32,
    img_height: i32,
    cam: Array3,
    pix00_loc: Array3, 
    delta_u: Array3,
    delta_v: Array3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, img_width: i32) -> Camera {
        let img_height = (img_width as f64 / aspect_ratio).floor() as i32;
        let img_height = if img_height < 1 { 1 } else { img_height };

        //camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
        let cam = Array3::zero();

        // define viewport vectors
        let vp_u = Array3::new([viewport_width, 0.0,0.0]);
        let vp_v = Array3::new([0.0, -viewport_height, 0.0]);

        // define delta vectors
        let delta_u = vp_u / img_width as f64;
        let delta_v = vp_v / img_height as f64;
        
        let viewport_upper_left = cam - Array3::new([0.0, 0.0, focal_length]) - vp_u/2.0 - vp_v/2.0;
        let pix00_loc = viewport_upper_left + 0.5*(delta_u + delta_v);

        Camera { 
            aspect_ratio,
            img_width,
            img_height,
            cam,
            pix00_loc, 
            delta_u,
            delta_v,
        }
    }

    fn ray_color(r: &Ray, world: &Vec<Box<dyn Hittable>>) -> Pixel {
        let record = hit_world(world, r, Interval::new(0.0, f64::INFINITY));

        if let Some(hit) = record {
            0.5*(hit.normal + Pixel::new([1.0, 1.0, 1.0]))
        } else {
            let unit_vec = r.direction.unit();
            let a = 0.5*(unit_vec[1] + 1.0);
            (1.0-a)*Pixel::new([1.0, 1.0, 1.0]) + a*Pixel::new([0.5, 0.7, 1.0])
        }
    }

    pub fn render(&self, world: &Vec<Box<dyn Hittable>>) -> io::Result<()> {
        let mut cout = io::stdout().lock();
        let mut cerr = io::stderr().lock();

        // render
        write!(&mut cout, "P3\n{} {}\n255\n", self.img_width, self.img_height)?;
        for j in 0..self.img_height {
            write!(&mut cerr, "Scanlines remaining: {}\n", self.img_height-j)?;
            cerr.flush()?;
            for i in 0..self.img_width {
                let pixel_loc = self.pix00_loc + (self.delta_u * i as f64) + (self.delta_v * j as f64);
                let ray_dir = pixel_loc - self.cam;
                let r = Ray::new(self.cam, ray_dir);
                let pix = Self::ray_color(&r, &world);

                write_pixel(&mut cout, pix)?;
            }
        }

        write!(&mut cerr, "Done\n")?;
        cerr.flush()?;
        Ok(())
    }
}
