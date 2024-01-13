fn main() {

    loop_and_panic(vec![1, 2, 3, 4, -5, 6]);

}

// NOTE: Functions

fn loop_and_panic(list_of_numbers: Vec<i32>) { 
    
    for value in list_of_numbers {

        if value < 0 {
            panic!("Negative number found");
        }
        println!("Number: {}", value);

    }

}
