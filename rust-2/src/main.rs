use std::io;
struct User{
    first_name:String,
    _last_name: String,
    _age:u8
}
struct Rectangle {
    width: u32,
    length: u32,
} 
impl Rectangle {
    fn area(&self)->u32{
        self.width*self.length
    }
    fn perimeter(&self)->u32{
        2*(self.width+self.length)
    }
    fn test()->u8{
        return 123;
    }
}
fn main() {
    let shape1 = Rectangle{width:30,length: 23};
    println!("The area of the reactangle is {}",shape1.area());
    println!("The Perimeter of the reactangle is {}",shape1.perimeter());
    // println!("The Test of the reactangle is {}",shape1.test()) // Error
    println!("The Test of the reactangle is {}",Rectangle::test());

    /*
    println!("Hello, World!");
    let user1 = User{
        first_name: String::from("Nilay"),
        _last_name: "Banerjee".to_string(),
        _age: 21
    };
    let user2 = input_user("Harkirat".to_string(),String::from("Singh"),32);
    println!("Hello {} & {}",user1.first_name,user2.first_name);
    */
    
    // println!("Hello {user1.first_name} & {}",user2.first_name);  // Filed Access isn't supported in formated string
   /* 
   let test_var:u8=123;
   println!("{test_var}");
   // println!("{test_var+2}");// Error
   println!("{}",test_var+46);
   input_number();
   */
}

fn _input_user(first_name:String, last_name:String, age:u8)->User{
    User{
        first_name,
        _last_name: last_name,
        _age: age
    }
}

fn _input_number(){
    println!("Enter A Number");
    let mut guessed_number = String::new();
    io::stdin().read_line(&mut guessed_number).expect("Failed To Read Line");
    println!("You Entered {guessed_number}");
}