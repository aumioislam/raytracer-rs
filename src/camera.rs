use std::io::{self, Write};

use crate::array3::Array3;
use crate::ray::*;
use crate::interval::*;
use crate::material::*;
use crate::util::*;

pub type Pixel = Array3;

pub struct Camera {
    img_width: i32,
    img_height: i32,
    cam: Array3,
    pix00_loc: Array3, 
    delta_u: Array3,
    delta_v: Array3,
    samples: i32,
    max_depth: i32,
    defocus_angle: f64,
    defocus_disk_u: Array3,
    defocus_disk_v: Array3,
}

impl Camera {
    pub fn new(
            aspect_ratio: f64,
            img_width: i32,
            samples: i32,
            max_depth: i32,
            vfov: f64,
            lookfrom: Array3,
            lookat: Array3,
            vup: Array3,
            defocus_angle: f64, 
            focus_dist: f64,
        ) -> Camera {
        let img_height = (img_width as f64 / aspect_ratio).floor() as i32;
        let img_height = if img_height < 1 { 1 } else { img_height };

        //camera
        let theta = degrees_to_rad(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
        let cam = lookfrom;

        //define basis vectors
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        // define viewport vectors
        let vp_u = viewport_width * u;
        let vp_v = viewport_height * -v;

        // define delta vectors
        let delta_u = vp_u / img_width as f64;
        let delta_v = vp_v / img_height as f64;
        
        let viewport_upper_left = cam - (focus_dist * w) - vp_u/2.0 - vp_v/2.0;
        let pix00_loc = viewport_upper_left + 0.5*(delta_u + delta_v);

        //camera defocus basis vectors
        let defocus_radius = focus_dist * degrees_to_rad(defocus_angle/2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera { 
            img_width,
            img_height,
            cam,
            pix00_loc, 
            delta_u,
            delta_v,
            samples,
            max_depth,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
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
                let mut pix = Array3::zero();

                for _ in 0..self.samples {
                    let r = self.get_ray(i, j);
                    pix = pix + self.ray_color(&r, world, self.max_depth);
                }

                write_pixel(&mut cout, pix, self.samples)?;
            }
        }

        write!(&mut cerr, "Done\n")?;
        cerr.flush()?;
        Ok(())
    }

    fn ray_color(&self, r: &Ray, world: &Vec<Box<dyn Hittable>>, depth: i32) -> Pixel {
        if depth == 0 { return Pixel::zero(); }

        let record = hit_world(world, r, Interval::new(0.001, f64::INFINITY));

        if let Some(hit) = record {
            let pair = hit.mat.scatter(r, &hit);
            if let Some((attenuation, scattered)) = pair {
                attenuation * self.ray_color(&scattered, world, depth-1)
            } else {
                Pixel::zero()
            }
        } else {
            let unit_vec = r.direction.unit();
            let a = 0.5*(unit_vec[1] + 1.0);

            (1.0-a)*Pixel::new([1.0, 1.0, 1.0]) + a*Pixel::new([0.5, 0.7, 1.0])
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pix00_loc + (self.delta_u * i as f64) + (self.delta_v * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        
        let ray_orig = if self.defocus_angle <= 0.0 { self.cam } else { self.defocus_disk_sample() };
        let ray_dir = pixel_sample - ray_orig;
        Ray { origin: ray_orig, direction: ray_dir }
    }

    fn pixel_sample_square(&self) -> Array3 {
        let pu = -0.5 + random_f64();
        let pv = -0.5 + random_f64();

        self.delta_u * pu + self.delta_v * pv
    }

    fn defocus_disk_sample(&self) -> Array3 {
        let p = Array3::random_in_unit_disk();
        self.cam + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

}
