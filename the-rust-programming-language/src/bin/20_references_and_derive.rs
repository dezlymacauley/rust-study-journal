#[derive(Debug, Clone, Copy)] // This is a list of trait a derive from
struct SomeStruct {
    num: i32,
}
// This will allow you to use {:?} in the print statement,
// to print out a Struct

// Clone explicitly makes a duplicate of a variable.
// E.g. some_struct.clone()

// Copy does the same thing as Clone but implicitly.
// E.g. some_struct  (No addition to the syntaxt)

fn print_some_struct(the_struct: &SomeStruct) {
        println!("{:?}", the_struct);
}

fn main() {
    let some_struct: SomeStruct = SomeStruct { num: 3 };


    // NOTE: In Rust when you pass a variable into a function, 
    // the function takes ownership of the memory for that variable.
    // To avoid this default behavior, use the `&` 
    // & is called an ampersand. 
    // It is a read-only link that points to an existing variable.

    print_some_struct(&some_struct);
    print_some_struct(&some_struct);
    println!("The value of num is: {} ", &some_struct.num);

}
