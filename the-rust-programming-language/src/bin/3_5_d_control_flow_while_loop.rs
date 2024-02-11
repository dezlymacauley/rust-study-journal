// NOTE: While loop

fn main() {

    let mut apples_remaining = 5;

    while apples_remaining != 1 {
        println!("Apples left: {}", apples_remaining);
        apples_remaining -= 1;
        // This is the same as saying 'apples_remaining = apples_remaining - 1'
    }

    println!("You only have {} apple left!", apples_remaining);

}
