use std::io;
fn main() {
    let mut name = String::new();
    println!("What is your name?...");
    match io::stdin().read_line(&mut name) {
        Ok(_) => {},
        Err(_) => {println!("Error");},
    }
    println!("Hello, {name}");
}
