fn main() {
    // 1
    let mut s = String::new();

    // 2
    let data = "initial contents".to_string();

    // 3
    let mut s = String::from("foo");
    s.push_str("bar");

    // 4
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // .push() only works for single characters
    println!("s2 is {}", s2);

    // 5
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 dies (rip)

    // 6
    let word = "Привет";
    for c in word.chars() {
        println!("{}", c);
    }
    for b in word.bytes() {
        println!("{}", b);
    }
}
