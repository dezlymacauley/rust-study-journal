// NOTE: Generics
// Generics help to prevent uneccessary code duplication.
// Generics can be used when you want to create a function,
// that can use the same logic to perform a task,
// even when then variable types for the arguements are different.

// A good way to think of generics is like a placeholder,
// for several varible types that fit in the same structure.
// E.g. Think of canned food. 
// Canned jam, and canned beans both have the same format.
// They both fit in a can, and both are opened by a can opener,
// however, what's inside is different.

// E.g. A function that can find the largest value in a list,
// regardless of whether that list of 32 bit integers, or a list of characters.

fn main() {

    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    
    
}

// NOTE: Putting <T> after a function name,
// shows that the function uses generics
// If you have one generic type, you call it the letter T
// The <T> stands for type
// The <T> variable type means "Whatever the variable type is"

// However the danger to this is that you could end up,
// trying to compare things that can't be compared.
// So you have to restrict to generic type a bit.
// This is done using a concept called traits.

// <T: PartilOrd + Copy> means that the type has be a variable type,
// that can be ordered or copied.

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest: T = number_list[0];
    
    for number in number_list {
        
        if number > largest {
            largest = number;
        }

    }
    largest  
}
