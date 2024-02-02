// NOTE: Statement and expressions
// Statements: Instructions that perform some action and do not return a value.
// Expressions: Evaluate to a resultant value.
// Expressions DO NOT end with a semicolon

fn main() {

    println!("{}", triple_plus_five(2));

}

// NOTE: Functions with a return value;
// This function returns a variable type of i32

fn triple_plus_five(_number: i32) -> i32 {
    // NOTE: This syntax is considered best practice in Rust
    (_number * 3) + 5 // This is an expression. Make sure there is no ; after.
    // In Rust the last expresion in a block is automatically returned.

    // NOTE: This syntax can also be written like this
    // Just make sure that if you do use the return keyword, you add the ;
    // return (_number * 3) + 5;
    // Not a fan of this syntax;
    
}
