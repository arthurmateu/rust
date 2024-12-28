fn main() {
    while_loop(5);

    let fib = [1, 1, 2, 3, 5, 8, 13];
    for element in fib.iter() {
        println!("The value is: {}", element);
    }

    range_thing(9);
}

//fn infinity() {
//    loop {
//        println!("again!");
//    }
//}

fn while_loop(mut x: i32) {
    while x != 0 {
        println!("{}", x);
        x = x - 1;
    }

    println!("End!");
}

fn range_thing(x: i32) {
    println!("Range!");
    // last element isn't inclusive
    for number in (1..x + 1).rev() {
        println!("{}", number);
    }
}
