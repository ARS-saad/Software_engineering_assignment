use crate::db::models::Doctor;

pub fn update_charge(doctor: &mut Doctor, charge: i32) {
    doctor.charge = charge;
    println!(
        "Doctor {} update the charge: {}",
        doctor.name, doctor.charge
    );
}
