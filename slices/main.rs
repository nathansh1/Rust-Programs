//string slices

fn main() {
    let s = String::from("Hello, world!");
    let hello = &s[0..5];
    let world = &s[7..];
    println!("{}, {}", hello, world);
}
