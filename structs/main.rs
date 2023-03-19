//using structs, impl blocks, and methods

fn main() {
    
    struct User {
		username: String,
		email: String,
		password: String
	}
	
	let user1 = User {
		email: String::from("nathanshadzeka@gmail.com"),
		username: String::from("shad"),
		password: String::from("***************")
	};
	
	println!("Email = {}", user1.email);
	println!("Username = {}", user1.username);
	println!("Password = {}", user1.password);
	
	
	
	
	
	/*let area = areaOfSquare(10);
	println!("Area is {}", area);*/
	
	let square1 = Square {
		side: 10
	};
	
	let area = area_of_square(&square1);
	let perim = perimeter_of_square(&square1);
	println!("Area of square is {}", area);
	println!("Perimeter of square is {}", perim);
	
	
	
	
	
	let rect1 = Rectangle{
		length: 20,
		width: 10
	};
	
	println!("Area of rectangle is {}", rect1.area());
	println!("Perimeter of rectangle is {}", rect1.perimeter());
	
}

/*fn area_of_square(side: u32) -> u32 {
	side*side
}

fn perimeter_of_square(side: u32) -> u32 {
	side*4
}*/

struct Square {
	side: u32
}

fn area_of_square(square: &Square) -> u32 {
	square.side*square.side
}

fn perimeter_of_square(square: &Square) -> u32 {
	square.side*4
}






struct Rectangle {
	length: u32,
	width: u32
}

impl Rectangle {
	
	fn area(&self) -> u32 {
		self.length*self.width
	}
	
	fn perimeter(&self) -> u32 {
		self.length*2+self.width*2
	}
	
}
