pub const TEST: [(&str, i32); 8] = [
    ("ECG", 400),
    ("X-Ray Chest (Digital)", 600),
    ("USG of Whole Abdomen", 2500),
    ("Echocardiogram", 2500),
    ("ETT", 3000),
    ("CT Scan", 6000),
    ("MRI", 8000),
    ("CBC", 400),
];

pub fn get_price(test_name: &str) -> i32 {
    for (name, price) in TEST {
        if name == test_name {
            return price;
        }
    }
    0
}
