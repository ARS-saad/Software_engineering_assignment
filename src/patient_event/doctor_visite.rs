use crate::db::models::Patient;

pub fn charge_for_patient(patient: Patient) {
    println!(
        "Doctor's visit for patient {}: {}",
        patient.name, patient.doctor.charge
    );
}
