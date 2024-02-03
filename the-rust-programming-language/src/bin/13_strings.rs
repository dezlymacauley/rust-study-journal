// NOTE: Strings

// NOTE: What is the difference between `String` and `&str`

/*

String:
    Ownership:
        String is a heap-allocated, growable, and mutable string type. 
        It owns its data, which means it is responsible for allocating,
        and deallocating memory.
        
    Mutability: String is mutable, 
    meaning you can modify the contents of the string after creation.
    

&str:
    Ownership:
        is a string slice, 
        which is a reference to a sequence of UTF-8 encoded bytes.
        It does not own the data; it's just a view into an existing string,
        owned by another data structure (like a String or an array).

    Mutability:
        &str is immutable. Once a string slice is created,
        you cannot change the contents of the underlying data.

*/

fn main() {

    // This is a String
    let my_string: String = String::new();

    // This is a String Slice
    let my_message: &str = "Hey it's Goku";
   
    // NOTE: How to convert a string slice to a Slice
    let converted_message: String = my_string.to_string();

}
