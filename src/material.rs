use crate::ray::*;
use crate::array3::*;
use crate::util::random_f64;

pub trait Scatterable {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Array3, Ray)>;
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatterable for Material {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Array3, Ray)> {
        match self {
            Material::Lambertian(l) => l.scatter(r, hr),
            Material::Metal(m) => m.scatter(r, hr),
            Material::Dielectric(e) => e.scatter(r, hr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Array3,
}

impl Scatterable for Lambertian {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Array3, Ray)> {
        let scatter_dir = hr.normal + Array3::random_unit();

        let scatter_dir = if scatter_dir.near_zero() { hr.normal } else { scatter_dir };

        let ray = Ray::new(hr.p, scatter_dir);
        let atten = self.albedo;
        Some((atten, ray))
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Array3, 
    pub fuzz: f64,
}

impl Scatterable for Metal {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Array3, Ray)> {
        let f = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };
        let reflection = reflect(&r.direction.unit(), &hr.normal);
        let ray = Ray::new(hr.p , reflection + f*Array3::random_unit());
        let atten = self.albedo;
        
        if ray.direction.dot(&hr.normal) > 0.0 {
            Some((atten, ray))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub index_r: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0-ref_idx) / (1.0+ref_idx);
        let r0 = r0*r0;
        
        r0 + (1.0 - r0)*(1.0-cosine).powf(5.0)
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Array3, Ray)> {
        let attn = Array3::new([1.0, 1.0, 1.0]);
        let refract_ratio = if hr.front_face { 1.0/self.index_r } else { self.index_r };

        let dir = r.direction.unit();

        let dp = hr.normal.dot(&(-dir));
        let cos_theta = if dp < 1.0 { dp } else { 1.0 };
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let vec_out = if refract_ratio*sin_theta > 1.0 || Self::reflectance(cos_theta, refract_ratio) > random_f64() {
            reflect(&dir, &hr.normal)
        } else {
            refract(&dir, &hr.normal, refract_ratio)
        };

        
        let ray = Ray::new(hr.p, vec_out);
        Some((attn, ray))
    }
}

