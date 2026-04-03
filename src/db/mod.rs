pub mod medical_test_data;
pub mod models;
use crate::db::models::{Doctor, Patient};

impl Doctor {
    pub fn new(id: u32, name: String, specialty: String, charge: i32) -> Self {
        println!("Doctor {} is arrive!", name);
        Self {
            id,
            name,
            specialty,
            charge,
        }
    }
}

impl Patient {
    pub fn new(id: u32, name: String, doctor: &Doctor) -> Self {
        println!("Patient {} is arrive for doctor {}!", name, doctor.name);
        Self {
            id,
            name,
            doctor: doctor.clone(),
            test: vec![],
        }
    }
}
