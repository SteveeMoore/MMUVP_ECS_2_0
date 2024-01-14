#![allow(dead_code)]

use nalgebra::{Matrix3, Vector6, Matrix6};

pub struct GradVComponent {
    tensor: Matrix3<f64>,
}

impl GradVComponent {
    pub fn new() -> Self {
        GradVComponent {
            tensor: Matrix3::zeros(),
        }
    }

    pub fn get_tensor(&self) -> Matrix3<f64> {
        self.tensor
    }

    pub fn set_tensor(&mut self, tensor: Matrix3<f64>) {
        self.tensor = tensor;
    }
}

pub struct DComponent {
    tensor: Matrix3<f64>,
    vector: Vector6<f64>,
}

impl DComponent {
    pub fn new() -> Self {
        let tensor = Matrix3::zeros();
        let vector = Vector6::zeros();
        DComponent { tensor, vector }
    }
    pub fn get_tensor(&self) -> Matrix3<f64> {
        self.tensor
    }
    pub fn get_vector(&self) -> Vector6<f64> {
        self.vector
    }
    pub fn set_vector(&mut self, vector: Vector6<f64>) {
        self.vector = vector;
        let matrix = Matrix3::new(
            vector[0], vector[3], vector[4], vector[3], vector[1], vector[5], vector[4], vector[5],
            vector[2],
        );
        self.tensor = matrix;
    }
    pub fn set_tensor(&mut self, tensor: Matrix3<f64>) {
        self.tensor = tensor;
        let vector = Vector6::new(
            tensor[(0, 0)],
            tensor[(1, 1)],
            tensor[(2, 2)],
            (tensor[(0, 1)] + tensor[(1, 0)]) / 2.0,
            (tensor[(0, 2)] + tensor[(2, 0)]) / 2.0,
            (tensor[(1, 2)] + tensor[(2, 1)]) / 2.0,
        );
        self.vector = vector;
    }
}

pub struct WComponent {
    tensor: Matrix3<f64>,
}

impl WComponent {
    pub fn new() -> Self {
        WComponent {
            tensor: Matrix3::zeros(),
        }
    }

    pub fn get_tensor(&self) -> Matrix3<f64> {
        self.tensor
    }

    pub fn set_tensor(&mut self, tensor: Matrix3<f64>) {
        let tensor = (tensor.transpose() - tensor) / 2.0;
        self.tensor = tensor;
    }
}

pub struct TrajectoryDeformationComponent {
    time: f64,
    grad_v: Matrix3<f64>,
}

impl TrajectoryDeformationComponent {
    pub fn new() -> Self {
        TrajectoryDeformationComponent{time: 0.0, grad_v: Matrix3::zeros()}
    }

    pub fn set_value(&mut self, time:f64, grad_v: Matrix3<f64>){
        self.time = time;
        self.grad_v = grad_v;
    }
    pub fn get_time(&self)-> f64{
        self.time
    }
    pub fn get_tensor(&self)->Matrix3<f64>{
        self.grad_v
    }
}

pub struct EpsComponent {
    tensor: Matrix3<f64>,
    vector: Vector6<f64>,
}

impl EpsComponent {
    pub fn new() -> Self {
        let tensor = Matrix3::zeros();
        let vector = Vector6::zeros();
        EpsComponent { tensor, vector }
    }
    pub fn get_tensor(&self) -> Matrix3<f64> {
        self.tensor
    }
    pub fn get_vector(&self) -> Vector6<f64> {
        self.vector
    }
    pub fn set_vector(&mut self, vector: Vector6<f64>) {
        self.vector = vector;
        let matrix = Matrix3::new(
            vector[0], vector[3], vector[4], vector[3], vector[1], vector[5], vector[4], vector[5],
            vector[2],
        );
        self.tensor = matrix;
    }
    pub fn set_tensor(&mut self, tensor: Matrix3<f64>) {
        self.tensor = tensor;
        let vector = Vector6::new(
            tensor[(0, 0)],
            tensor[(1, 1)],
            tensor[(2, 2)],
            (tensor[(0, 1)] + tensor[(1, 0)]) / 2.0,
            (tensor[(0, 2)] + tensor[(2, 0)]) / 2.0,
            (tensor[(1, 2)] + tensor[(2, 1)]) / 2.0,
        );
        self.vector = vector;
    }
}

pub struct SigmaComponent {
    tensor: Matrix3<f64>,
    vector: Vector6<f64>,
}

impl SigmaComponent {
    pub fn new() -> Self {
        let tensor = Matrix3::zeros();
        let vector = Vector6::zeros();
        SigmaComponent { tensor, vector }
    }
    pub fn get_tensor(&self) -> Matrix3<f64> {
        self.tensor
    }
    pub fn get_vector(&self) -> Vector6<f64> {
        self.vector
    }
    pub fn set_vector(&mut self, vector: Vector6<f64>) {
        self.vector = vector;
        let matrix = Matrix3::new(
            vector[0], vector[3], vector[4], vector[3], vector[1], vector[5], vector[4], vector[5],
            vector[2],
        );
        self.tensor = matrix;
    }
    pub fn set_tensor(&mut self, tensor: Matrix3<f64>) {
        self.tensor = tensor;
        let vector = Vector6::new(
            tensor[(0, 0)],
            tensor[(1, 1)],
            tensor[(2, 2)],
            (tensor[(0, 1)] + tensor[(1, 0)]) / 2.0,
            (tensor[(0, 2)] + tensor[(2, 0)]) / 2.0,
            (tensor[(1, 2)] + tensor[(2, 1)]) / 2.0,
        );
        self.vector = vector;
    }
}

pub struct SigmaRateComponent {
    tensor: Matrix3<f64>,
    vector: Vector6<f64>,
}

impl SigmaRateComponent {
    pub fn new() -> Self {
        let tensor = Matrix3::zeros();
        let vector = Vector6::zeros();
        SigmaRateComponent { tensor, vector }
    }
    pub fn get_tensor(&self) -> Matrix3<f64> {
        self.tensor
    }
    pub fn get_vector(&self) -> Vector6<f64> {
        self.vector
    }
    pub fn set_vector(&mut self, vector: Vector6<f64>) {
        self.vector = vector;
        let matrix = Matrix3::new(
            vector[0], vector[3], vector[4], vector[3], vector[1], vector[5], vector[4], vector[5],
            vector[2],
        );
        self.tensor = matrix;
    }
    pub fn set_tensor(&mut self, tensor: Matrix3<f64>) {
        self.tensor = tensor;
        let vector = Vector6::new(
            tensor[(0, 0)],
            tensor[(1, 1)],
            tensor[(2, 2)],
            (tensor[(0, 1)] + tensor[(1, 0)]) / 2.0,
            (tensor[(0, 2)] + tensor[(2, 0)]) / 2.0,
            (tensor[(1, 2)] + tensor[(2, 1)]) / 2.0,
        );
        self.vector = vector;
    }
}

pub struct ElasticityTensorComponent {
    value: Matrix6<f64>,
}

impl ElasticityTensorComponent {
    pub fn new() -> Self {
        let value = Matrix6::zeros();
        ElasticityTensorComponent { value }
    }

    pub fn print_matrix(&self) {
        let matrix = self.value;
        for i in 0..matrix.nrows() {
            for j in 0..matrix.ncols() {
                print!("{:12}", matrix[(i, j)]);
            }
            println!();
        }
    }

    pub fn set_value(&mut self, value: Matrix6<f64>) {
        self.value = value;
    }

    pub fn get_value(&self) -> Matrix6<f64> {
        self.value
    }
}