use crate::db::models::Doctor;
use crate::db::models::Patient;

impl Patient {
    pub fn change_doctor(&mut self, new_doctor: Doctor) -> &Self {
        println!(
            "Changing doctor for patient {} to {}",
            self.name, new_doctor.name
        );
        self.doctor = new_doctor;
        self
    }
}
