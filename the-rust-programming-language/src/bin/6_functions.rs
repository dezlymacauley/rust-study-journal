// NOTE: Functions

fn main() {

    // In Rust, you can use a function before it is declared.
    say_hello("Dezly");

    add_two_numbers(10, 18);

}

// This is how to declare a function
fn say_hello(_username: &str) {
    println!("Hello {}", _username);
}

fn add_two_numbers(_num1: i32, _num2: i32) {
    let total: i32 = _num1 + _num2;
    println!("{_num1} + {_num2} = {}", total);
}

// NOTE: What's the difference between parameters and arguements?

// When you are declaring a function, 
// the values in the bracket are called parameters.
// Think of parameters as a template,
// for what type of values a function will accept.

// When a function is used, the values that are submitted into the brackets,
// are called arguements. 
// Arguments are the actual input that a function requires,
// to perform an action.
