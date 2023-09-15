use crate::array3::Array3;

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
