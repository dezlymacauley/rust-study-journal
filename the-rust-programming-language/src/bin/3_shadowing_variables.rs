// NOTE: Shadowing and scope
// Shadowing is when you create a new variable using the `let` keyword,
// that has the same name as an existing variable.

fn main() {
    let cars = 2;
    let books: u32 = 8;
    println!("The number of cars is {cars}"); // cars = 2 
    println!("The number of books is {books}"); // books = 8 
            
    
    {
        let cars = cars + 3;
        println!("The number of cars is {cars}"); 
        // cars = 5 within this scope
        
        // NOTE: When shadowing, because you are creating a new variable,
        // (only the name of the previous variable is copied),
        // you are free to use a different variable type if you want.
        // E.g. The original was a u32 but in this scope it is a &str type

        let books: &str = "red and green books"; 
        println!("I have the {books}"); 
        // I have the red and green books
    }
        
    println!("The number of cars is {cars}"); // cars is back to 2 again.
    println!("The number of books is {books}"); // books = 8

}
