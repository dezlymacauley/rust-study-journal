fn main() {

    // NOTE: In Rust, variables are immutable by default  
    // If you want to create a variable with a value that can be changed,
    // you need to add the key word `mut`

    let mut my_number = 5;
    println!("My number is {my_number}"); 
    // This will print out My number is 5
    
    my_number = 10;
    println!("My number is {my_number}"); 

}
