use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // cannot borrow `list` as immutable because it is also borrowed as mutable
    // println!("After defining closure: {list:?}"); // Will error
    borrows_mutably();
    println!("After calling closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
