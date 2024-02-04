// NOTE: Error Handling

/*
Rust groups errors into two major categories: 
recoverable and unrecoverable errors.

NOTE: Recoverable Error:

For a recoverable error, such as a file not found error, 
we most likely just want to report the problem to the user,
and retry the operation.

For a recoverable error, Rust has the type `Result<T, E>`

NOTE: Unrecoverable errors: 

Are always symptoms of bugs, like trying to access a location beyond,
the end of an array, and so we want to immediately stop the program.

For an unrecoverable error, there is a `panic!` macro that stops execution,
when the program encounters an unrecoverable error.

*/

// This line below is not related to error checking.
// This line allows you to use File type functionality.

use std::fs::File;


fn main() {

    // How to make the program panic on purpose
    // panic!("crash and burn");

let greeting_file_result: Result<File, std::io::Error> 
    = File::open("hello.txt");

// In Rust, the File::open In Rust, the File::open function returns a Result type. The Result type is an enum with two variants: Ok and Err. The Ok variant contains the successfully opened file, while the Err variant contains an error value indicating why the file couldn't be opened.
// function returns a Result type.
// The Result type is an enum with two variants: Ok and Err.
// The Ok variant contains the successfully opened file,
// while the Err variant contains an error value,
// indicating why the file couldn't be opened.

    let greeting_file: File = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

}
