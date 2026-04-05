use crate::db::models::Patient;

impl Patient {
    pub fn add_test(&mut self, test: Vec<String>) {
        self.test.clear();
        self.test.extend(test);
        println!(
            "D. {} give the test {:?} for patient {}!",
            self.doctor.name, self.test, self.name
        );
    }
}
