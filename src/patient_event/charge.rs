use crate::db::models::Patient;

pub fn charge_for_patient(patient: Patient) {
    println!(
        "Charging for patient {}: {}",
        patient.name, patient.doctor.charge
    );
}
