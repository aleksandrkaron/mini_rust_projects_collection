use std::io;

fn main() {
    println!("Enter a temperature in Celsius.");

    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Failed to read temperature");

    let temp: i32 = temp.trim().parse().expect("Not a valid number");

    println!("The temperature is: {temp}.");


}
