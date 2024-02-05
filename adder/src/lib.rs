// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// NOTE: What is #[derive(Debug)]
/*

In Rust, the #[derive(Debug)] attribute is used to automatically implement,
the Debug trait for a struct. The Debug trait allows you to use the {:?}
format specifier with the println! and format! macros,
making it easier to print or format debugging information for your types.

*/


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     // let result = add(2, 2);
    //     // assert_eq!(result, 4);
    //     assert_eq!(2 + 2, 4);

        // NOTE: How does assert_eq! Work?
        // The assert_eq! macro expects two arguments separated by a comma: 
        // the actual value and the expected value. 
        // E.g (2 + 2, 4) means check if (2 + 2) = 4
    
        // NOTE: How to run your test 
        // `cargo test`

    // }
    
    // #[test]
    // fn failing_test() {
    //     panic!("Make this test fail");
    // }
    #[test]
    fn larger_can_hold_smaller() {
        let larger: Rectangle = Rectangle{
            width: 8,
            height: 7,
        };
        let smaller: Rectangle = Rectangle{
            width: 5,
            height: 1,
        };
    }
}
