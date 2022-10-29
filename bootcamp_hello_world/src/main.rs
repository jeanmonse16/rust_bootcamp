// const MAX: i16 = 2;
// static MIN: i16 = 1;

fn main() {
    println!("Hello, world!");
    //creation
    let a: i32 = 5;
    //mutability
    let mut b: i32 = 6;
    b = 7;
    //shadowing
    let c: i32 = 8;
    let c: i32 = 10;
    println!("{}", c);
    //scope
    {
        let d = 3;
        println!("{}", d);
    }
    let e: i32 = my_function(2);
    println!("{}", e)

}

fn my_function (a: i32) -> i32 {
    let b = a + 2;
    println!("b is equal to {}", b);
    b
}
