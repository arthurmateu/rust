fn main() {
    let number = if true { 8 } else { -8 }; // Has to be same type in both cases

    if number > 0 {
        println!("Positive number.");
    } else if number < 0 {
        println!("Negative number.");
    } else {
        println!("Number is zero.");
    }
}
