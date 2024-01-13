
// NOTE: This Debug attribute is used when you want to print out a struct.
// As in the entire struct, and not just a part of it.
#[derive(Debug)]

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    super_power: Option<String>, 
    // NOTE: Option means that this String could be "None"
    code_name: Option<String>,
}

fn main() {

    // NOTE: This will print out the entire struct
    println!("{:?}", 
        Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            age: 25,
            super_power: None,
            code_name: None,
        }
    
    );

    // NOTE: How to create a variable from the struct template
    let alfredo = Person {
        first_name: "Alfredo".to_string(),
        last_name: "Sanchez".to_string(),
        age: 25,
        super_power: None,
        
        // NOTE: If a field in the struct is an `Option` type,
        // then when you use it in a variable you need to wrap it in,
        // Some().
        // This is because `Option` means that the value could be none.
        code_name: Some("Blitz".to_string()),
    };
    
    // NOTE: How to acces the values of a variable created using a Struct
    println!("The person's first name is: {}", alfredo.first_name);
    println!("The person's super power is: {:?}", alfredo.super_power);

}
