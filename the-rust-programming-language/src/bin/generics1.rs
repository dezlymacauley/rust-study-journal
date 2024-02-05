// NOTE: Generics 1 - The non-optimised version of the code

// This is the starting code
// No generics are used here

fn main() {
    
    //
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    
    let mut largest = &number_list[0];
    // NOTE: About the `&`(This symbol is called ampersand)

    // This is because I don't want the ownership of the original vector,
    // to be transfered to the variable largest. 
    // I just want a reference to the original
    // So think of a reference as a read-only link to a variable.
    // In this example &number_list is a read-only link,
    // to the original vector &number_list
   
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest numbes is {}", largest);

}
