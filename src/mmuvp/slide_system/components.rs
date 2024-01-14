#![allow(dead_code)]

use nalgebra::{Vector3, Matrix3};

pub struct BurgersVectorComponent {
    vectors: Vec<Vector3<f64>>,
}

impl BurgersVectorComponent {
    pub fn new() -> Self {
        let mut vectors = Vec::with_capacity(24);
        for _ in 0..24 {
            vectors.push(Vector3::new(0.0, 0.0, 0.0));
        }
        BurgersVectorComponent { vectors }
    }

    pub fn set_vector(&mut self, index: usize, vector: Vector3<f64>) {
        if index < self.vectors.len() {
            self.vectors[index] = vector;
        }
    }

    pub fn get_vector(&self, index: usize) -> Option<&Vector3<f64>> {
        if index < self.vectors.len() {
            Some(&self.vectors[index])
        } else {
            None
        }
    }
}

pub struct NormalVectorComponent {
    vectors: Vec<Vector3<f64>>,
}

impl NormalVectorComponent {
    pub fn new() -> Self {
        let mut vectors = Vec::with_capacity(24);
        for _ in 0..24 {
            vectors.push(Vector3::new(0.0, 0.0, 0.0));
        }
        NormalVectorComponent { vectors }
    }

    pub fn set_vector(&mut self, index: usize, vector: Vector3<f64>) {
        if index < self.vectors.len() {
            self.vectors[index] = vector;
        }
    }

    pub fn get_vector(&self, index: usize) -> Option<&Vector3<f64>> {
        if index < self.vectors.len() {
            Some(&self.vectors[index])
        } else {
            None
        }
    }
}

pub struct BNComponent {
    matrixs: Vec<Matrix3<f64>>,
}

impl BNComponent {
    pub fn new()->Self{
        let mut matrixs = Vec::with_capacity(24);
        for _ in 0..24 {
            matrixs.push(Matrix3::identity());
        }
        BNComponent { matrixs }
    }

    pub fn set_matrix(&mut self, index:usize, matrix: Matrix3<f64>){
        if index < self.matrixs.len(){
            self.matrixs[index]=matrix;
        }
    }
    pub fn get_matrix(&self, index: usize) -> Option<&Matrix3<f64>> {
        if index < self.matrixs.len() {
            Some(&self.matrixs[index])
        } else {
            None
        }
    }
}

pub struct TauComponent {
    values: Vec<f64>,
}

impl TauComponent {
    pub fn new() -> Self {
        let values = vec![0.0; 24];
        TauComponent { values }
    }

    pub fn set_values(&mut self, index: usize, value: f64) {
        if index < self.values.len() {
            self.values[index] = value;
        }
    }

    pub fn get_values(&self, index: usize) -> Option<f64> {
        if index < self.values.len() {
            Some(self.values[index])
        } else {
            None
        }
    }   
}

pub struct TauRateComponent {
    values: Vec<f64>,
}

impl TauRateComponent {
    pub fn new() -> Self {
        let values = vec![0.0; 24];
        TauRateComponent { values }
    }

    pub fn set_values(&mut self, index: usize, value: f64) {
        if index < self.values.len() {
            self.values[index] = value;
        }
    }

    pub fn get_values(&self, index: usize) -> Option<f64> {
        if index < self.values.len() {
            Some(self.values[index])
        } else {
            None
        }
    }   
}


pub struct GammaComponent {
    values: Vec<f64>,
}

impl GammaComponent {
    pub fn new() -> Self {
        let values = vec![0.0; 24];
        GammaComponent { values }
    }

    pub fn set_values(&mut self, index: usize, value: f64) {
        if index < self.values.len() {
            self.values[index] = value;
        }
    }

    pub fn get_values(&self, index: usize) -> Option<f64> {
        if index < self.values.len() {
            Some(self.values[index])
        } else {
            None
        }
    }   
}

pub struct GammaRateComponent {
    values: Vec<f64>,
}

impl GammaRateComponent {
    pub fn new() -> Self {
        let values = vec![0.0; 24];
        GammaRateComponent { values }
    }

    pub fn set_values(&mut self, index: usize, value: f64) {
        if index < self.values.len() {
            self.values[index] = value;
        }
    }

    pub fn get_values(&self, index: usize) -> Option<f64> {
        if index < self.values.len() {
            Some(self.values[index])
        } else {
            None
        }
    }   
}

pub struct HVectorComponent{
    vector:Vec<f64>,
}

impl HVectorComponent{
    pub fn new() -> Self {
        let vector = vec![0.0; 24];
        //println!("{:?}", vector);
        HVectorComponent{ vector }
    }

    pub fn set_vector(&mut self, index: usize, value: f64) {
        if index < self.vector.len() {
            self.vector[index] = value;
        }
    }

    pub fn get_value(&self, index: usize) -> Option<f64> {
        if index < self.vector.len() {
            Some(self.vector[index])
        } else {
            None
        }
    }
}

pub struct HMatrixComponent{
    matrix:Vec<Vec<f64>>,
}

impl HMatrixComponent{
    pub fn new() ->Self {
        let mut matrix:Vec<Vec<f64>> = Vec::with_capacity(24);
        for _ in 0..24{
            let vector = vec![0.0; 24];
            matrix.push(vector);
        }
        HMatrixComponent{matrix}
    }

    pub fn set_value(&mut self, index_i:usize, index_j:usize, value:f64){
        if index_i < self.matrix.len() && index_j<self.matrix[index_i].len(){
                self.matrix[index_i][index_j]=value;
        }
    }
    pub fn get_value(&self, index_i:usize, index_j:usize) -> Option<f64>{
        if index_i<self.matrix.len(){
            if index_j<self.matrix[index_i].len(){
                Some(self.matrix[index_i][index_j])
            } else {
                None
            }
        } else {
            None
        }
    }
}
