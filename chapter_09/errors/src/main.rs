use core::panic;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // 1.
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Tried to create file but there was a problem: {:?}", e)
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    // 2.
    let f = File::open("hello.txt").unwrap();

    // 3.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// 4.
// use std::fs::File;   <- Was imported previously
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt"); // an '?' here is equivalent to the match expression below

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        // an '?' here too!
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } // ...and since this doesn't include a ';', it will be the 'return' of the function!
}

// 5. (equivalent to 4.)
//fn read_username_from_file() -> Result<String, io::Error> {
//    let mut s = String::new();
//
//    File::open("hello.txt")?.read_to_string(&mut s)?;
//
//    Ok(s)
//}
