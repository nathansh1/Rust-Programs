enum IPVariants {
    IPV4(String),
    IPV6(String),
}

enum Message {
    Write(String),
    Color(i32,i32,i32),
    Move{x:i32,y:i32},
    Quit,
}

impl Message {
    fn call(&self){
        println!("I'm inside the implementation");
    }
}

fn main() {
    let _ip1 = IPVariants::IPV4(String::from("192.168.0.1"));
    let _ip2 = IPVariants::IPV6(String::from("::1"));

    let _variable = Message::Write(String::from("HI"));
    //variable.call();

    let name = String::from("Nathan");

    println!("Char at position 9 is {}", 
            match name.chars().nth(9) {
                Some(v) => v.to_string(),
                None => "No character at this position".to_string(),
            });

    println!("Char at position 3 is {}",
             match name.chars().nth(3) {
                 Some(v) => v.to_string(),
                 None => "No character at this position".to_string(),
             });



}
