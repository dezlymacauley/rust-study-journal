use std::io::{BufReader, BufRead};

// NOTE: How to create a library package in rust

// Method 1: `cargo init --lib .`
// First create a folder using `mkdir name-of-folder`
// Next `cd name-of-created folder`
// This will create a the project in an existing folder

// Method 2: `cargo new --lib name-of-your-new-folder`
// This will create a new project folder

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input line");
    line.trim().to_string()
}

