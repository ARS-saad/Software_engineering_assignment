use crate::db::models::Patient;

impl Patient {
    pub fn add_test(&mut self, test: Vec<String>) {
        self.test.clear();
        self.test.extend(test);
        println!(
            "Doctor {} give the parient {} for test {:?}!",
            self.doctor.name, self.name, self.test
        );
    }
}
