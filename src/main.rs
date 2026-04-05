// Import module
pub mod db;
pub mod doctor_event;
pub mod patient_event;

// Use modules
use crate::db::models::{Doctor, Patient};
use crate::doctor_event::update_charge::update_charge;
use crate::patient_event::doctor_visite::charge_for_patient;
use crate::patient_event::pay_bill::pay_bill;
use crate::patient_event::total_test_cost::total_test_cost;

fn main() {
    // Create doctors and patients
    let mut doctor1 = Doctor::new(
        1,
        "Jarir Mohammad Jubaier".to_string(),
        "Neuro sergeon".to_string(),
        700,
    );
    let doctor2 = Doctor::new(
        2,
        "Ismail Hossain Parvaz".to_string(),
        "Neuro sergeon".to_string(),
        800,
    );
    let patient1 = Patient::new(1, "Sara".to_string(), &doctor1);
    update_charge(&mut doctor1, 1000);
    let mut patient2 = Patient::new(2, "Nasib".to_string(), &doctor1);

    charge_for_patient(patient1.clone());
    pay_bill(patient1);

    charge_for_patient(patient2.clone());

    patient2.change_doctor(doctor2);
    charge_for_patient(patient2.clone());

    patient2.add_test(vec!["CBC".to_string(), "MRI".to_string()]);
    println!("Total cost for Test: {}", total_test_cost(&patient2.test));
    pay_bill(patient2);
}
