fn ownership() {

    let numbers: Vec<i32> = vec![1, 2, 3];
    let slice: &[i32] = &numbers[..]; //creates a slice of all elements in numbers
    println!("slice = {:?}", slice);

}

fn modifiable() {
    let mut numbers: Vec<i32> = vec![1, 2, 3];
    let slice: &mut [i32] = &mut numbers[..];
    slice[0] = 10;
    println!("slice = {:?}", slice);

}


fn main() {

    ownership();
    modifiable();

}
