#[derive(Clone)]
pub struct Doctor {
    pub id: u32,
    pub name: String,
    pub specialty: String,
    pub charge: i32,
}

#[derive(Clone)]
pub struct Patient {
    pub id: u32,
    pub name: String,
    pub doctor: Doctor,
    pub test: Vec<String>,
}
