///working with structs, and impl blocks
///feb 15 2023

fn main() {
	
	let mut nate = User::new(String::from("Nathan Shadzeka"), 16, 113.5);
	println!("{}", User::name(&nate));
	println!("{}", User::age(&nate));
	println!("{}", User::weight(&nate));
	User::set_age(&mut nate, 17);
	println!("{}", User::age(&nate));
	User::set_weight(&mut nate, 120.0);
	println!("{}", User::weight(&nate));
}

pub struct User {
	name: String,
	age: u32,
	weight: f32,
}

impl User {
	pub fn new(name: String, age: u32, weight: f32) -> Self {
		return User {name, age, weight}
	}

	pub fn name(&self) -> &str {
		return &self.name
	}

	pub fn age(&self) -> u32 {
		return self.age
	}

	pub fn weight(&self) -> f32 {
		return self.weight
	}

	pub fn set_age(&mut self, new_age: u32) {
		self.age = new_age;
	}

	pub fn set_weight(&mut self, new_weight: f32) {
		self.weight = new_weight;
	}
}
