// NOTE: The Option Enum

fn main() {

    // NOTE: You don't need to type this in order to use.
    // As a matter of fact... You SHOULD NOT type this out.
    // The Option enum and its variants are included in Rust by default.
    // E.g. Same as how you don't need to do anything special to use println!

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("a string");
    
    let absent_number: Option<&i32> = None;
    
    // NOTE: x is an integer, y is an optional integer
    // optional integer means that it is posible for the variable y,
    // to not have a value.
    let x: i8 = 5;
    let y: Option<i8> = Some(5); 
    // let y: Option<i8> = None; 
    // y is = to 5 but y could also not have a value
    
    
    // NOTE: Adding an integer type and an optional intege type.
    let sum_of_x_and_y = x + y.unwrap_or(7);
    // This means that if y is None then use the default value of 7,
    // If y is not None then use the value inside of Some
    println!("x is {}, y is {:?} and the total is {}", x, y, sum_of_x_and_y);








}
