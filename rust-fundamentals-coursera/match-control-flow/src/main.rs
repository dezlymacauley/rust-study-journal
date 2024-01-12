use::std::io;

fn main() {

    println!("Please enter a greeting: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    // .trim is used to remove any empty spaces from the user's response
    match name.trim() {
        "Good Bye" => println!("Sorry to see you go"),
        "Hello" => println!("Hi, nice to meet you!"),
        _ => println!("Malevolent Kitchen"),
    }

    // NOTE: _ => Means "For all other posssibilities, do this"  

}
