fn main() {
    println!("Hello, world!");
    my_first_function(2.64, 21);
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    println!("y is {}", y);

    let a_number = five(5);
    println!("the number is {}", a_number);

    let mut number = 3;
    let conditional_number = if true { 5 } else { 6 };

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    while number < 5 {
        number = number + 1;
    };

    loop {
        if number > 10 {
            break;
        }
        number = number + 1
    }
}
// to return something with dont use the semicolon at the end of the function
fn five(x: i32) -> i32 {
    5 + x
}

fn my_first_function(x: f64, y:u8) {
    println!("hello there {}, i'm {}", x, y);
}
