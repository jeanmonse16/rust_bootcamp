use std::io;

fn insert_temperature_unit() -> String {
    let mut unit = String::new();
    println!("Input your initial temperature unit!, C for Celsius; F for fahrenheit");
    io::stdin()
      .read_line(&mut unit)
      .invalid_input("")
      .expect("failed while reading temperature unit");
      println!("{}",unit);

    unit.trim().to_string()
}

fn main() {
    println!("Hello, world!");
    let mut temperature_unit = insert_temperature_unit();

    while temperature_unit != "C" || temperature_unit != "F" {
        println!("Insert a valid temperature unit");
        let mut temperature_unit = insert_temperature_unit();
    }

    println!("temperature unit is {}", temperature_unit);

}
