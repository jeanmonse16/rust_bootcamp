fn main() {
    let s1: String = String::from("Rust");

    let s2: String = s1.clone(); // s2 is new owner of rust string
    let s3 = generate_string();
    let s4 = add_to_string(s2);
    print_string(s1.clone());
    println!("Hello, world!: {}", s1);
    println!("Hello s3 {}", s3);
    println!("{}", s4);

    let x = 10;
    let y = x;
    print_integer(x);
    println!("x is: {}", x);
}

fn print_integer (int: i32) {
    println!("int is: {}", int)
}

fn add_to_string (mut p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}

fn generate_string () -> String {
    String::from("verris")
}

fn print_string (p1: String) {
    println!("{}",p1)
}
