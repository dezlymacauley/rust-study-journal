// NOTE: Data Types

// Rust is a statically typed language, 
// which means that it must know the types of all variables at compile time. 
// The compiler can usually infer what type we want to use based on the value, 
// and how we use it.

// NOTE: Scalar Types

// Rust has four primary scalar types: 
// integers, 
// floating-point numbers, 
// Booleans, 
// and characters.


// NOTE: Integer Types
// An integer is a number without a fractional component
// In other words a whole number

// i means that it is a signed integer. 
// The number can be positive or negative.

// u means unsigned integer
// The value cannot have a a sign. 
// I.e. The value can only be positive

/*

NOTE: Integer sizes - Signed (Used to store negative and postivite numbers)

+--------+--------+-------------------------------+
| Length | Signed | Range of Numbers (Inclusive)  |
+--------+--------+-------------------------------+
|  8-bit |   i8   |         -128 to 127            |
| 16-bit |  i16   |       -32,768 to 32,767        |
| 32-bit |  i32   |  -2,147,483,648 to 2,147,483,647|
| 64-bit |  i64   | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807|
|128-bit | i128   | (extremely large range)        |
|  arch  | isize  | (platform-dependent)           |
+--------+--------+-------------------------------+

******************************************************************************/



/*

NOTE: Unsigned Integer sizes (For storing positive numbers)

+--------+---------+-------------------------------+
| Length | Unsigned| Range of Numbers (Inclusive)  |
+--------+---------+-------------------------------+
|  8-bit |    u8   |            0 to 255           |
| 16-bit |   u16   |           0 to 65,535          |
| 32-bit |   u32   |       0 to 4,294,967,295       |
| 64-bit |   u64   | 0 to 18,446,744,073,709,551,615|
|128-bit |  u128   | (extremely large range)        |
|  arch  |  usize  | (platform-dependent)           |
+--------+---------+-------------------------------+

// NOTE: How are these calculated

Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive,
where n is the number of bits that variant uses.

So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. 
Unsigned variants can store numbers from 0 to 2n - 1,
so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

Additionally, the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

// NOTE: How to know which integer size to choose?
// integer types default to i32 and u32
// The primary situation in which you’d use isize or usize,
// is when indexing some sort of collection.


*/


fn main() {

    // NOTE: Boolean (true or false)
    // Booleans are 1 byte in size
    let is_a_rust_developer: bool = true;
    println!("Is he a Rust Developer? {is_a_rust_developer}");
    
    let is_korean: bool = false;
    println!("Is he Korean? {is_korean}");

    // NOTE: Character types
    // A char type is 4 bytes in size
    // WARNING. Char literals are declared using single quotes
    // Don't use double quotes (That's for strings)
    let fruit_emoji: char = '🍑';
    println!("My favourite fruit is {fruit_emoji}");

    let my_favorite_kanji: char = '暖';
    println!("My favourite kanji is {my_favorite_kanji}");
   
    // NOTE: Integers

    let my_number: i32 = 50;
    let my_number2 = 50i32; // This is another syntax. Not a fan of it.
    let my_number3: u32 = 90_000; 
    // You can use an underscore to make things easir to read in your code.
    // Note that this will print out 90000 (not 90_000)
    println!("{my_number}");
    println!("Another way {my_number2}");
    println!("{my_number3}");


    // NOTE: Floating-point numbers (Numbers that have a decimal point part)
    // Rust has two types of floating points:
    // All floating-point types are signed.        
    // f32 = 32 bits = range is -2,147,483,648 to 2,147,483,647
    // f64 = 64 bits = range is -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    
    // Floating-point numbers are represented according to the IEEE-754 standard. 
    // The f32 type is a single-precision float, and f64 has double precision.
    
    // The default type is f64 because on modern CPUs, 
    // it’s roughly the same speed as f32 but is capable of more precision. 
    
    let shipping_fee: f32 = 2.67;
    let scientific_precise_number: f64 = 2.7277727272;
    println!("{shipping_fee}");
    println!("{scientific_precise_number}");

}
