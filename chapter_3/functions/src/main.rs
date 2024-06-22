fn main() {
    println!("Hello, world!");

    another_function(2, 4);

    println!(
        "Here's a function with a return value of {}.",
        double_val(8)
    );
}

fn another_function(x: i32, y: i32) {
    println!("The value of 'x' is {}, while 'y' is {}.", x, y);
}

fn double_val(x: i32) -> i32 {
    x * 2
}
