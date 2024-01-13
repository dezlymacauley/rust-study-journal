// NOTE: Comments that start with `//!` must appear first
// These types of comments are for documenting the whole package.

//! This is a library that provides utilities for command line tools. 
//! So far it only provides an option to read from stdin

use std::io::{BufReader, BufRead};

// NOTE: How to document your code
// /// In Rust, three forward slashes are a special type of comment for documentation.

/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message,
/// "Failed to read inptu line."
/// # Examples:
/// ```
/// let input = read_stdin();
/// ```

// NOTE: How to use this documentation

// `cd` to the project directory
// Run the command `cargo doc`
// This will create a folder in /target/doc/document_your_code
// There will be an index.html file that allows you view the documentation that you wrote. 

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input line");
    line.trim().to_string()
}

