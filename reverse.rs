///messing with string objects and references
///reversing them (obv)
///feb 22 2023

fn main() {
	println!("{}", reverse_string(String::from("hello")));
	println!("{}", reverse_string(String::from("rust")));
	println!("{}", reverse_string(String::from("")));
	println!("{}", reverse_stringref("nathan"));
	println!("{}", reverse_stringref("shadzeka"));
	println!("{}", reverse_stringref("lol"));
}

fn reverse_string(s: String) -> String {
	s.chars().rev().collect()
}


fn reverse_stringref(s: &str) -> String {
	s.chars().rev().collect()
}
