
///working with functions, returns, doubles, and integers, different number types

fn main() {
	println!("{}", production_rate_per_hour(4));
	println!("{}", production_rate_per_hour(9));
	println!("{}", production_rate_per_hour(1));
	println!("{}", production_rate_per_hour(7));
	println!("{}", production_rate_per_hour(0));
	println!("{}", working_items_per_minute(5));
	println!("{}", working_items_per_minute(8));
	println!("{}", working_items_per_minute(1));
	println!("{}", working_items_per_minute(10));
	println!("{}", working_items_per_minute(0));
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
	if speed <= 4 { return speed as f64*221.}
	if speed >= 5 && speed <= 8 { return speed as f64*221.*0.9}
	if speed >=9 { return speed as f64*221.*0.77}
	return 0.
}

pub fn working_items_per_minute(speed: u8) -> u32 {
	return production_rate_per_hour(speed) as u32/60
}

