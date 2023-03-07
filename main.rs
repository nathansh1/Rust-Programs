/* functions
 * reusable code
 * for organization
 */

fn main() {
    f1();
    f2(2);
    let s = sum(1,2);
    println!("The sum = {}", s);
}

fn f1(){ //no parameter
	println!("I'm f1");
}

fn f2(a:i32){ //with parameter
	println!("The value of a is: {}", a);
}

fn sum(a:i32, b:i32)->i32{ //with parameters and return value
	a+b
}
