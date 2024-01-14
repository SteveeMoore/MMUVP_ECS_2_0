#![allow(dead_code)]
use nalgebra::Matrix3;

pub struct RotationComponent{
    tensor_form: Matrix3<f64>,
}

impl RotationComponent{
    pub fn new() -> RotationComponent{
        RotationComponent{
            tensor_form: Matrix3::identity(),
        }
    }

    pub fn set_matrix(&mut self, matrix: Matrix3<f64>) -> Result<(), &str> {
        // Проверка на ортогональность матрицы
        if Self::is_orthogonal(&matrix) {
            self.tensor_form = matrix;
            Ok(())
        } else {
            Err("Матрица не является ортогональной.")
        }
    }

    pub fn get_tensor(&self) -> Matrix3<f64> {
        self.tensor_form
    }

    fn is_orthogonal(matrix: &Matrix3<f64>) -> bool {
        let transposes = matrix.transpose();
        let identity = matrix * transposes;
    
        identity.is_identity(1e-5)
    }
}

pub struct RotationRateComponent{
    tensor_form: Matrix3<f64>,
}

impl RotationRateComponent{
    pub fn new() -> RotationRateComponent{
        RotationRateComponent{
            tensor_form: Matrix3::identity(),
        }
    }

    pub fn set_tensor(&mut self, matrix: Matrix3<f64>) -> Result<(), &str> {
        // Проверка на ортогональность матрицы
        if Self::is_orthogonal(&matrix) {
            self.tensor_form = matrix;
            Ok(())
        } else {
            Err("Тензор не является ортогональным.")
        }
    }

    pub fn get_tensor(&self) -> Matrix3<f64> {
        self.tensor_form
    }

    fn is_orthogonal(matrix: &Matrix3<f64>) -> bool {
        let transposes = matrix.transpose();
        let identity = matrix * transposes;
    
        identity.is_identity(1e-5)
    }
}

pub struct SpinComponent{
    tensor_form: Matrix3<f64>,
}

impl SpinComponent{
    pub fn new() -> SpinComponent{
        SpinComponent{
            tensor_form: Matrix3::identity(),
        }
    }

    pub fn set_tensor(&mut self, tensor: Matrix3<f64>) -> Result<(), &str> {
        // Проверка на ортогональность матрицы
        if Self::is_orthogonal(&tensor) {
            self.tensor_form = tensor;
            Ok(())
        } else {
            Err("Тензор не является ортогональным.")
        }
    }

    pub fn get_tensor(&self) -> Matrix3<f64> {
        self.tensor_form
    }

    fn is_orthogonal(matrix: &Matrix3<f64>) -> bool {
        let transposes = matrix.transpose();
        let identity = matrix * transposes;
    
        identity.is_identity(1e-5)
    }
}