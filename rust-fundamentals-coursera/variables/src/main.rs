fn main() {

    let message = "Name: Alfredo, Weight: ";
    let weight = 190.0;
    
    // NOTE: In Rust you can't divide an integer by a float
    
    // Both numbers must have decimals 

    let kilos = weight / 2.2;
    println!("{}{}", message, weight);
    println!("{}{}", message, kilos);

    let mut height = 189;

    println!("{}", height);
    height = 174;
    println!("{}", height);
}
