use std::io;

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
    println!("Hello, world!");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];
    let c = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
"August", "September", "October", "November", "December"];

println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

    let element = a[index];

    println!(
    "The value of the element at index {} is: {}",
    index, element
    );
}
