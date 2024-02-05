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

// NOTE: `&mut` is how you create a mutable references 

fn mutate_struct(the_struct: &mut SomeStruct) {
    the_struct.num = 5 
}

fn main() {
    let mut some_struct: SomeStruct = SomeStruct { num: 3 };


    // NOTE: In Rust when you pass a variable into a function, 
    // the function takes ownership of the memory for that variable.
    // To avoid this default behavior, use the `&` 
    // & is called an ampersand. 
    // It is a read-only link that points to an existing variable.

    print_some_struct(&some_struct);
    println!("The value of num is: {} ", &some_struct.num);
    
    mutate_struct(&mut some_struct);
    println!("The value of num is: {} ", &some_struct.num);

    // NOTE: In Rust you can have an unlimited number of references,
    // but you are only allowed to have one mutable reference at a time.

}
