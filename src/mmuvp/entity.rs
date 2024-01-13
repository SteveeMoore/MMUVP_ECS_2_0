#![allow(dead_code)]
#[derive(Eq, Hash, PartialEq, Clone)]
pub struct CrystalEntity {
    id: u32,
}

impl CrystalEntity {
    pub fn new(id: u32) -> Self {
        CrystalEntity { id }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
