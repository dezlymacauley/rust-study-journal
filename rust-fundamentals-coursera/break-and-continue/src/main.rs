fn main() {
    
    for value in 1..=10 {

        if value % 2 == 0 {
            // This will skip all of the even numbers in the range.
            continue;
        }
        println!("value is {}", value);

        if value == 7 {
            // Exit the loop when the value is 7
            break;
        }

    }

}
