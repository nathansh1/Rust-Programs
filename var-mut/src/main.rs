///practicing cargo and basic variable mutability

fn main() {
    let mut a = 1; //without 'mut', theres an error
    println!("The value of a is {}", a);
    a = 2;
    println!("The value of a is {}", a);
}
