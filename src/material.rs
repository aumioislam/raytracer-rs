use crate::ray::*;
use crate::array3::*;

pub trait Scatterable {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Array3, Ray)>;
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Scatterable for Material {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Array3, Ray)> {
        match self {
            Material::Lambertian(l) => l.scatter(r, hr),
            Material::Metal(m) => m.scatter(r, hr),
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
        let ray = Ray::new(hr.p , reflection + self.fuzz*Array3::random_unit());
        let atten = self.albedo;
        
        if ray.direction.dot(&hr.normal) > 0.0 {
            Some((atten, ray))
        } else {
            None
        }
    }
}
