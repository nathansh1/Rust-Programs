/* if or else-if statements
 * loops
 * stuff like that
 * literally ... controlling the flow of the code
 */

fn main() {
    
    println!("\n ----IFs---- \n");
    
    let a = 10;
    if a<20 {
		println!("{} is less than 20", a);
	} else if a>20 {
		println!("{} is more than 20", a);
	} else {
		println!("{} is 20", a);
	}
	
	
	println!("\n ----LOOPs---- \n");
	/*loop {
		println!("Eating a burger wit no honey mustard🗿");
	}*/ 
	// ^^ infinite loop ^^
	
	loop {
		println!("Eating a burger wit no honey mustard🗿");
		break;
	}
	
	let mut b = 3;
	
	while b!=0 {
		println!("{}",b);
		b-=1;
	}
	
	let c = [0,1,2,3,4,5];
	print!("The array's contents are: ");
	for value in c.iter() {
		print!("{} ", value);
	}
	
    
}
