// NOTE: Enums

fn main() {
    
    // NOTE: Step 1: Define the enum
     
    enum Color {
        Yellow,
        Red,
        Green,
        Blue,
    }

    // NOTE: Rust can attach functions to enums
    // impl means implementation
    // You can attach multiple functions

    impl Color {

        fn green_part(&self) -> bool {
            match self {
                Color::Yellow => true,
                Color::Blue => true,
                _ => false, // This means for all other cases, return false.
            }
        }
        
        fn is_green(&self) -> bool {
            if let Color::Green = self {
                return true;
            }
            return false;
        }


    }

    /*
    NOTE: Explanation
    
    The function is called green_part. 
    It takes in a reference to the enum

    */


    // NOTE: Step 2: 
    // Create a function with a match statement that uses the enum
    
    // This function prints a colour
    // In order for this function to work it needs "color" as an arguement.
    // The variable type of _colour is Color (which is an enum)
    // Because the variable type is an enum, 
    // that means that there only a limit number of options: Red, Green, Blue
    fn print_color(_color: Color) {
       match _color {
            Color::Red => println!("red"),
            Color::Green => println!("green"),
            Color::Yellow => println!("yellow"),
            Color::Blue => println!("blue"),
        } 
    
    }


//=============================================================================


}
