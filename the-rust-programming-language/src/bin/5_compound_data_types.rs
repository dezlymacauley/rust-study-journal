// NOTE: Compound Data Types
// Rust has two primitive compound types: tuples and arrays.

fn main() {

    // NOTE: Tuple
    // Tuples have a fixed length: Once declared, 
    // they cannot grow or shrink in size.
    // The variables in a tuple can be of different types.
    // The values of a tuple are stored on the stack

    // How to declare a tuple
    let player_info: (&str, u32, bool) = ("Cindy", 30, true);

    // How to access values in a tuple
    let name: &str = player_info.0;    
    let age: u32 = player_info.1;    
    let is_adult: bool = player_info.2; 

    println!("Player name is {}", name);
    println!("Player age is {}", age);
    println!("Is the player an adult? {}", is_adult);

//=============================================================================

    // NOTE: Arrays
    // Arrays have a fixed length.
    // They cannot grow or shrink after being created.
    // The variables stored in an array must be of the same type.
    // The value of an array is stored in the stack.
    
    // How to declare the variable type of an array
    // [u32; 4] means the array will store 4 values that are u32
    let player_scores: [u32; 4] = [30, 50, 60, 20];
    
    // How to access elements of an array
    let first_number: u32 = player_scores[0];
    let last_number: u32 = player_scores[3];
    
    println!("The first number is {}", first_number);
    println!("The last number is {}", last_number);


}
