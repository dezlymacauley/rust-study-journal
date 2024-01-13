// NOTE: This is known as a `string slice`
fn print_str(s: &str) {
    println!("{}", s);

    let mut new_string = s.to_string();
    println!("{}", new_string);

    new_string.push_str("adding more to my last message");
}


// NOTE: This is known as a `String type`
fn print_string(s: String) {
    println!("{}", s);
}

fn main() {

    let s: &str = "hello, world";
    print_str(s);

    let salutation: String = String::from("hello");
    print_string(salutation);
    
}
