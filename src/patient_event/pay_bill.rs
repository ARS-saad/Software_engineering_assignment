use crate::{
    db::{medical_test_data::TEST, models::Patient},
    patient_event::total_test_cost::total_test_cost,
};

pub fn pay_bill(patient: Patient) {
    let test_cost = total_test_cost(&patient.test);
    let doctor_charge = patient.doctor.charge;
    let total_cost = test_cost + doctor_charge;
    println!();
    println!("Hospital charge for {}", patient.name);
    for i in patient.test {
        println!("{}: {}", i, TEST.iter().find(|t| t.0 == i).unwrap().1);
    }
    println!(
        "D. {} visite: {}",
        patient.doctor.name, patient.doctor.charge
    );
    println!("----------------------");
    println!("Total cost: {}", total_cost);
    println!();
}
