// NOTE: Structs (or Structure)
// A custom data type that lets you package together,
// and name multiple related values that make up a meaningful group.

fn main() {
    
    // Step 1: Create the template for the struct
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Step 2: Create an instance of that struct
    let user1 = User {
        active: true,
        username: String::from("Bob"),
        email: String::from("rtr@gmail.com"),
        sign_in_count: 4,
    };

}
