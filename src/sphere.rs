use crate::array3::Array3;
use crate::ray::*;
use crate::interval::*;
use crate::material::*;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Array3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Array3, radius: f64, material: Material) -> Sphere {
        Sphere{ center, radius, material, }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, inter: Interval) -> Option<HitRecord> {
        let along_centers = r.origin - self.center;
        
        let a = r.direction.square();
        let h = r.direction.dot(&along_centers);
        let c = along_centers.square() - self.radius*self.radius;

        let discrim = h*h - a*c;

        if discrim < 0.0 {
            None
        } else {
            let sqrtd = discrim.sqrt();
            let r1 = (-h - sqrtd)/a;
            let r2 = (-h + sqrtd)/a;

            let root = if inter.contains(r1) {
                Some(r1)
            } else if inter.contains(r2) {
                Some(r2)
            } else {
                None
            };

            if let Some(t) = root {
                let p = r.at(t);
                let normal = (p - self.center) / self.radius;
                
                let front_face = r.direction.dot(&normal) < 0.0;
                let normal = if front_face { normal } else { -normal };

                Some(HitRecord{ p, normal, t, front_face, mat: &self.material })
            } else {
                None
            }
        }
    }
}
