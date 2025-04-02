use std::io;
fn main() {
    println!("Hello, world!");
    let test_var:u8=123;
    println!("{test_var}");
    // println!("{test_var+2}");// Error
    println!("{}",test_var+46);
    input_number();
}
fn input_number(){
    println!("Enter A Number");
    let mut guessed_number = String::new();
    io::stdin().read_line(&mut guessed_number).expect("Failed To Read Line");
    println!("You Entered {guessed_number}");
}