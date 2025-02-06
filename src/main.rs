use std::io;

fn main() {
    println!("Enter a temperature in Celsius.");

    let mut temp_in_c = String::new();

    io::stdin().read_line(&mut temp_in_c).expect("Failed to read temperature");

    let temp_in_c: f64 = temp_in_c.trim().parse().expect("Not a valid number");

    let temp_in_f = temp_in_c * 1.8 + 32.0;

    println!("{temp_in_c}°C in Fahrenheit is: {temp_in_f}°F.");

}
