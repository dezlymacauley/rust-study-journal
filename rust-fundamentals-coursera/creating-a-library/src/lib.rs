// NOTE: How to create a library package in rust

// Method 1: `cargo init --lib .`
// First create a folder using `mkdir name-of-folder`
// Next `cd name-of-created folder`
// This will create a the project in an existing folder

// Method 2: `cargo new --lib name-of-your-new-folder`
// This will create a new project folder

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
