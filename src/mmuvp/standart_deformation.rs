#![allow(dead_code)]

use nalgebra::Matrix3;

pub fn simple_shear(l12:f64) -> Matrix3<f64>{
    Matrix3::new(
        0.0, l12, 0.0, 
        -l12, 0.0, 0.0, 
        0.0, 0.0, 0.0)
}

pub fn uniaxial_tension(l11:f64)->Matrix3<f64>{
    Matrix3::new(
        l11, 0.0, 0.0, 
        0.0, -l11/2.0, 0.0, 
        0.0, 0.0, -l11/2.0)
}