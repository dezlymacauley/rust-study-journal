fn main() {

    let mut vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_string = String::from("Hello, world!");

    own_integer(my_int);
    println!("{}", my_int);

    own_string(&my_string);
    println!(":?", my_string);

    own_vec(&my_vec);
}

fn own_vec(mut vector: Vec<i32>) {
    vector.push(10);
    println!("{:?}",vector);
}

fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: String) {
    println!("{}", s);
}
