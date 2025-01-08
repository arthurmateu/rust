fn main() {
    let some_option_value: Option<i32> = None;
    if let Some(x) = some_option_value {
        // If x is not none, does the action below
        println!("{x}");
        // Note that this code is only useful for refutable patterns, otherwise the
        // compiler will bicker about it
    } else {
        println!("Nope.");
    }
}
