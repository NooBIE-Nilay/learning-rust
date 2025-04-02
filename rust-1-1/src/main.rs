fn main() {
 let message: &str = "Hello From";
 let name = String::from("NooBIE");
 println!("{} {}",message,name);
 println!("Length of \"{}\" is {}",message,get_string_length(&message));
 println!("Length of \"{}\" is {}",name,get_string_length(&name));
 string_test();
}
fn _is_even(number: i32)-> bool {
    if number % 2 == 0{
        return  true;
    }
    return false;
}

fn get_string_length(s: &str)->usize{
    s.chars().count() // REMEMBER: No Semicolon 
    // equivalent to
    // return  s.chars().count();
}

fn string_test(){
    // So Basically rust treats equal strings as equal and stores them in same pointers
    let a:&str = "abc";
    let b: &str = "abc";
    let c: &str = "xyz";
    let d:String= String::from("abc");
    let  mut e: &str = "abc";
    // a="xyz"; //Error
    println!("&a->{}\t&b->{}\t&c->{}\t&d->{}\t&e->{}",&a,&b,&c,&d,&e);
    e="xyz";
    println!("a->{}\tb->{}\tc->{}\td->{}\te->{}",a,b,c,d,e);
    // if a==b{
    //     println!("ab Equal");
    // }else {
    //     println!("ab Not Equal");
    // }
    // if &a==&b{
    //     println!("&a&b Equal Pointers");
    // }else {
    //     println!("&a&b Not Equal Pointers");
    // }
    // if a==c{
    //     println!("ac Equal");
    // }else{
    //     println!("ac Not Equal");
    // }
    // if a==d{
    //     println!("ad Equal");
    // }else{
    //     println!("ad Not Equal");
    // }
    // if &a==&d{
    //     println!("&a&d Equal");
    // }else{
    //     println!("&a&d Not Equal");
    // }
}