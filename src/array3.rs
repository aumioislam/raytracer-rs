use std::ops::{Add, Sub, Div, Mul, Neg, Index};
use std::cmp::PartialEq;

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
