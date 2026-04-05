use crate::db::medical_test_data::get_price;

pub fn total_test_cost(test: &Vec<String>) -> i32 {
    let mut cost = 0;
    for t in test {
        cost += get_price(&t);
    }
    cost
}
