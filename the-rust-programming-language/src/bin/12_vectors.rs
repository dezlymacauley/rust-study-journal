// NOTE: Vectors
// Vectors can grow and shrink (You can add and remove values stored)
// However all of the values in a vector must be of the same type.
// vector is freed when it goes out of scope

fn main() {
    
    // NOTE: How to create an empty Vector

    // let empty_vector: Vec<i32> = Vec::new(); 

    // Rust conveniently provides the vec! macro,
    // which will create a new vector that holds the values you give it. 

    // How to create a Vector with values inside
    let mut cool_vector: Vec<i32> = vec![10, 51, 18, 9];

    // NOTE: How to add items to the end of a vector
    cool_vector.push(19);

    // NOTE: How to read a value from a Vector
    // Pay attention to the `&` symbol
    
    // Since I am only reading from the Vector,
    // I used `&` before the name. & means that this is a reference,
    // to the actual value stored in the vector.

    // Avoiding Ownership Transfer: By using a reference,
    // you avoid transferring ownership of the value from the vector.
    // This is particularly important,
    // if you need to use the value later in the program,
    // or if you want to keep the vector intact after the operation.

    println!("{}", &cool_vector[4]);

    // NOTE: How to loop over elements in a Vector


    let lucky_numbers: Vec<i32> = vec![16, 18, 20, 30];

    for value in &lucky_numbers {
       
        println!("{}", value);  

    } 
    
}
