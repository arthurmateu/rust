struct Point {
    x: i32,
    y: i32,
}

#[warn(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // Literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"), // Anything with _ is default case (ignores).
    }

    println!("===");
    // Named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        // Some(y) shadows x, because it creates a pattern. If written instead as:
        //
        //  Some(n) if n == y => ..
        //
        // It will avoid the shadowing
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    println!("===");
    // Multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    println!("===");
    // Range of values (example uses int, but also works with char values)
    // You can also use @ to bind
    // variable_name @ [range]
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        // '..' also works for ignoring remaining parts
        _ => println!("something else"),
    }

    println!("===");
    // Destructuring structs. Logic also works for tuples
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p; // Shorthand for { x:x, y:y }
                            // (read as x assigns to variable a, y to variable b)
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    println!("===");
    // Destructuring enums
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            // Logic also works for nested structs/enums
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }
}
