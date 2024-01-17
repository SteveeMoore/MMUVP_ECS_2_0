#![allow(dead_code)]
pub struct GrainSizeComponent{
    value:f64
}
impl GrainSizeComponent{
    pub fn new()->Self{
        GrainSizeComponent{value: 0.0}
    }
    pub fn set_value(&mut self, value:f64){
        self.value = value;
    }
    pub fn get_value(&self)->f64{
        self.value
    }
}
pub struct AccumEnergyComponent{
    value: f64,
}

impl AccumEnergyComponent{
    pub fn new()->Self{
        AccumEnergyComponent{value: 0.0}
    }
    pub fn set_value(&mut self, value:f64){
        self.value = value;
    }
    pub fn get_value(&self)->f64{
        self.value
    }
}
pub struct AccumEnergyRateComponent{
    value: f64,
}

impl AccumEnergyRateComponent{
    pub fn new()->Self{
        AccumEnergyRateComponent{value: 0.0}
    }
    pub fn set_value(&mut self, value:f64){
        self.value = value;
    }
    pub fn get_value(&self)->f64{
        self.value
    }
}

pub struct SubGrainsComponent{
    vector: Vec<f64>,
}
impl SubGrainsComponent{
    pub fn new()->Self {
        SubGrainsComponent{vector: Vec::new()}
    }
    pub fn set_value(&mut self, index: usize, value: f64) {
        if index < self.vector.len() {
            self.vector[index] = value;
        }
    }
    pub fn push_value(&mut self, value:f64){
        self.vector.push(value);
    }
    pub fn get_value(&self, index: usize) -> Option<f64> {
        if index < self.vector.len() {
            Some(self.vector[index])
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        self.vector.len()
    }
}

pub struct DriveForceRecrComponent{
    vector: Vec<f64>
}
impl DriveForceRecrComponent{
    pub fn new()->Self {
        DriveForceRecrComponent{vector:Vec::new()}
    }
    pub fn set_value(&mut self, index: usize, value: f64) {
        if index < self.vector.len() {
            self.vector[index] = value;
        }
    }
    pub fn push_value(&mut self, value:f64){
        self.vector.push(value);
    }
    pub fn get_value(&self, index: usize) -> Option<f64> {
        if index < self.vector.len() {
            Some(self.vector[index])
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        self.vector.len()
    }
}

pub struct DriveForceRecrCrystComponent{
    value: f64
}
impl DriveForceRecrCrystComponent{
    pub fn new()->Self{
        DriveForceRecrCrystComponent{value: 0.0}
    }
    pub fn set_value(&mut self, value:f64){
        self.value = value;
    }
    pub fn get_value(&self)->f64{
        self.value
    }
}

pub struct FacetMobilityComponent{
    value: f64,
}

impl FacetMobilityComponent{
    pub fn new()->Self{
        FacetMobilityComponent{value: 0.0}
    }
    pub fn set_value(&mut self, value:f64){
        self.value = value;
    }
    pub fn get_value(&self)->f64{
        self.value
    }
}

pub struct VelocityFacetComponent{
    value: f64
}
impl VelocityFacetComponent{
    pub fn new()->Self{
        VelocityFacetComponent{value: 0.0}
    }
    pub fn set_value(&mut self, value:f64){
        self.value = value;
    }
    pub fn get_value(&self)->f64{
        self.value
    }
}