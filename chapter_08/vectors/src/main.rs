fn main() {
    // 1.
    let v: Vec<i32> = Vec::new();

    // 2.
    let v = vec![1, 2, 3];

    // 3.
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 4.
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // When accessing an out-of-bounds item, it will panic
    let third: Option<&i32> = v.get(2); // When accessing out-of-bounds, it will return None

    // 5.
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // 6.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // 7.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
} // v goes out of scope!
