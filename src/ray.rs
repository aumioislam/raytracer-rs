use crate::array3::Array3;
use crate::interval::Interval;
use crate::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Array3,
    pub direction: Array3,
}

impl Ray {
    pub fn new(origin: Array3, direction: Array3) -> Ray {
        Ray { origin, direction, }
    }

    pub fn at(&self, t: f64) -> Array3 {
        self.origin + (self.direction * t)
    }
}

pub struct HitRecord<'m> {
    pub p: Array3,
    pub normal: Array3,
    pub t: f64,
    pub front_face: bool,
    pub mat: &'m Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, vals: Interval) -> Option<HitRecord>;
}

pub fn hit_world<'m> (world: &'m Vec<Box<dyn Hittable>>, r: &Ray, mut vals: Interval) -> Option<HitRecord<'m>> {
    let mut record = None;
    
    for obj in world {
        if let Some(hit) = obj.hit(r, vals) {
            vals.max = hit.t; 
            record = Some(hit);
        }
    }

    record
}
