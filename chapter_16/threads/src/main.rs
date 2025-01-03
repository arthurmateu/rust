use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..11 {
            println!("Spawned Thread = {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..6 {
        println!("Main Thread = {i}");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // code does not continue until thread finishes execution

    let v = vec![3, 2, 1];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    // drop(v);

    handle.join().unwrap();
}
