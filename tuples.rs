///experimenting with tuples
fn main() {
	
	let tup: (i32,i32,i32) = (1,2,3);
	let (x,y,z) = tup;
	println!("x = {}, y = {}, and z = {}", x, y, z);
	
	let a = tup.0;
	println!("a = {}", a);
}