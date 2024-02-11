fn main() {

    let mut apples_picked: i32 = 0;
    
    loop {
        apples_picked +=1;

        if apples_picked == 5 {
            println!("Task completed: {} apples have been picked", apples_picked);
            break;
        } else if apples_picked == 1 {
            println!("{} apple picked so far...", apples_picked);
        } else {
            println!("{} apples picked so far...", apples_picked);
        }

    }

}
