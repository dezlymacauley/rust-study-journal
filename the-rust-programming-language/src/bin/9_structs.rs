// NOTE: Structs (or Structure)
// A custom data type that lets you package together,
// and name multiple related values that make up a meaningful group.

fn main() {
    
    // Step 1: Create the template for the struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // NOTE: Step 2: Create an instance of that struct
    
    // If you want to create an instance of a struct,
    // with fields that can be modified later (e.g. changing the email),
    // then you must add the keyword `mut` to the variable name.
    // You can't simply make one field mutable

    let mut user1 = User {
        username: String::from("Bob"),
        email: String::from("rtr@gmail.com"),
        sign_in_count: 4,
        active: true,
    };

    // How to store the value of a field in a struct into a variable
    let name_of_user = user1.username;
    println!("{name_of_user}");

    // How to change the value of a field in a struct
    user1.active = false;
    println!("{}", user1.active);

    // NOTE: Instead of manually creating instances from the struct,
    // you can create a function that will create new instances.

    fn build_user(email: String, username: String) -> User {
        
        // If a field has the same name as an arguement in the function,
        // insted of email: email,
        // You can simplify this to just email.
        // This is called the field init, shorthand syntax.

        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
     
    }

    // NOTE: How to use this function to create a new user
    let user2: User = build_user(
        String::from("max@gmail.com"),
        String::from("Max"),
    ); 
    

}
