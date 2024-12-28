fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // some stuff about data types and basic arithmetic stuff. btw i got yelled by the compilor
    // to add an underscore for some reason. will hopefully appear on the book why
    let _neat_separator: f64 = 100_000.2;

    // should probably look into this as a great way of initializing variables (possibly on one
    // line or something??)
    let x: (u8, f32) = (3, 9.9);
    let _eight_bit = x.0; // 3
    let _thirtytwo_float = x.1; // 9.9

    // arrays are boring (self-limiting type and size)
    let _boring = [5, 3, 2, 1];
    let _two = _boring[2]; // 2
}
