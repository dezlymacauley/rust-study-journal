// NOTE: Lifetimes
// Think of lifetimes adding a saftey check to a function and its inputs,
// to make sure that the inputs do not go out of scope before some neccessary,
// calculation or return happens.

// 1. The are only relevant for references.
// Lifetimes make sure that a reference never points to some memory,
// that has been freed.
//
// 2. They don't change the lifetimes of the parameters. 
// The concept of lifetimes only adds checks to the parameters.

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

// NOTE: <'a> is how you use the Lifestyle functionality
// What this means is that struct1 and struct2 needs to stay in scope,
// at least as long as the duration of this function.
// This helps to prevent situations where you have dangling references.

// NOTE: What is a dangling reference?
// This is a reference that points to invalid data

fn biggest<'a>(struct1: &'a SomeStruct,
    struct2: &'a SomeStruct) -> &'a SomeStruct {
    
    if struct1.num > struct2.num {
        struct1 // If struct1 is biggest then it will return struct 1
    } else {
        struct2
    }

    // NOTE: What is the point of `Lifetimes`?
    // This is the compilers way of making sure that the arguements,
    // struct 1 and struct 2, 
    // do not go out of scope before the function biggest is able to compare,
    // and return one of them.

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
    
    let bigger: SomeStruct;


    
}
