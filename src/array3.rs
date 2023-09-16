use std::ops::{Add, Sub, Div, Mul, Neg, Index};
use std::cmp::{PartialEq, min};

use crate::util::{random_f64, random_rge_f64};

#[derive(Debug, Clone, Copy)]
pub struct Array3 {
    elems: [f64; 3],
}

impl Array3 {
    pub fn zero() -> Array3 {
        Array3 { elems: [0.0, 0.0, 0.0,] } 
    }

    pub fn new(elems: [f64; 3]) -> Array3 {
        Array3 { elems }
    }

    pub fn random() -> Array3 {
        Array3::new([random_f64(), random_f64(), random_f64()])
    }

    pub fn random_rge(min: f64, max: f64) -> Array3 {
        Array3::new([random_rge_f64(min, max), random_rge_f64(min, max), random_rge_f64(min, max)])
    }

    fn random_in_unit_sphere() -> Array3 {
        loop {
            let p = Self::random_rge(-1.0, 1.0);
            if p.norm() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit() -> Array3 {
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_on_hemisphere(normal: &Array3) -> Array3 {
        let v = Self::random_unit();
        if v.dot(normal) > 0.0 {
            v
        } else {
            -v
        }
    }

    pub fn dot(&self, rhs: &Array3) -> f64 {
        self.elems.iter().zip(rhs.elems.iter())
            .map(|(x1, x2)| x1 * x2)
            .sum()
    }

    pub fn cross(&self, rhs: &Array3) -> Array3 {
        Array3::new([
            self[1]*rhs[2] - self[2]*rhs[1],
            self[2]*rhs[0] - self[0]*rhs[2],
            self[0]*rhs[1] - self[1]*rhs[0],
        ])
    }

    pub fn square(&self) -> f64 {
        self.dot(self)
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn unit(&self) -> Array3 {
        *self / self.norm()
    }

    pub fn near_zero(&self) -> bool {
        self[0].abs() < f64::EPSILON && self[1].abs() < f64::EPSILON && self[2].abs() < f64::EPSILON
    }
}

pub fn reflect(vec: &Array3, normal: &Array3) -> Array3 {
    *vec - 2.0*vec.dot(normal)*(*normal)
}

pub fn refract(vec: &Array3, normal: &Array3, eta_ratio: f64) -> Array3 {
    let dp = normal.dot(&(-(*vec)));
    let cos_theta = if dp < 1.0 { dp } else { 1.0 };
    let r_out_perp = eta_ratio * (*vec + cos_theta*(*normal));
    let r_out_parallel = -((1.0 - r_out_perp.square()).abs().sqrt()) * (*normal);
    r_out_perp + r_out_parallel
}

impl Index<usize> for Array3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl Neg for Array3 {
    type Output = Array3;

    fn neg(self) -> Self::Output {
        Array3 { elems: [-self[0], -self[1], -self[2],] }
    }
}

impl Add for Array3 {
    type Output = Array3;

    fn add(self, rhs: Self) -> Self::Output { 
        Array3 { elems: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2],] }
    }
}

impl Sub for Array3 { 
    type Output = Array3;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<Array3> for Array3 { 
    type Output = Array3;

    fn mul(self, rhs: Array3) -> Self::Output {
        Array3 { elems:
            [
                self[0] * rhs[0],
                self[1] * rhs[1],
                self[2] * rhs[2],
            ]
        }
    }
}

impl Mul<f64> for Array3 { 
    type Output = Array3;

    fn mul(self, rhs: f64) -> Self::Output {
        Array3 { elems:
            [
                self[0] * rhs,
                self[1] * rhs,
                self[2] * rhs,
            ]
        }
    }
}

impl Mul<Array3> for f64 {
    type Output = Array3;

    fn mul(self, rhs: Array3) -> Self::Output {
        rhs * self
    }
}

impl Div<Array3> for Array3 { 
    type Output = Array3;

    fn div(self, rhs: Array3) -> Self::Output {
        Array3 { elems:
            [
                self[0] / rhs[0],
                self[1] / rhs[1],
                self[2] / rhs[2],
            ]
        }
    }
}

impl Div<f64> for Array3 {
    type Output = Array3;

    fn div(self, rhs: f64) -> Self::Output {
        Array3 {
            elems: [self[0]/rhs, self[1]/rhs, self[2]/rhs,]
        }
    }
}

impl PartialEq for Array3 {
    fn eq(&self, other: &Self) -> bool {
        self[0] == other[0] && self[1] == other[1] && self[2] == other[2]
    }
}
