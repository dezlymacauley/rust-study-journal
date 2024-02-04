// NOTE: Generics
// Generics help to prevent uneccessary code duplication.
// Generics can be used when you want to create a function,
// that can use the same logic to perform a task,
// even when then variable types for the arguements are different.

// E.g. A function that can find the largest value in a list,
// regardless of whether that list of 32 bit integers, or a list of characters.

fn main() {

    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];

}
