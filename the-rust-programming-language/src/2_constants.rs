// NOTE: Constants
// In Rust Constants are always immutable.
// You can use `mut` and `const` at the same time.
// The Data type of a constant must always be annotated.

fn main() {
    const MONTHS_IN_A_YEAR: u32 = 12;
    println!("There are {MONTHS_IN_A_YEAR} months in a year");
}

// u32 is the data type, 
// u stands for unsigned, that means that the value can only be positive, 
// 32 stands 32-bit integer.
