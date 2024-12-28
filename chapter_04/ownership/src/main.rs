fn main() {
    // experimenting a bit with ownership
    let s1 = String::from("Hello");
    println!("{}", s1);

    // let s2 = s1.push_str(", world!");
    // println!("{}", s2); doesn't work

    let mut s2 = s1.clone();
    s2.push_str(", world!");
    println!("{}", s2);
}
