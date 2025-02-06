use std::io;

fn main() {
    loop {
        println!("\n>>> Select a program to run. <<<\n\n\
        1) Celsius to Fahrenheit Converter\n\
        2) Fahrenheit to Celsius Converter\n\
        3) Project 3\n\
        4) Project 4");
        // celsius_to_fahrenheit();

        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nInvalid option");
                continue;
            },
        };

        match option {
            1 => {
                celsius_to_fahrenheit();
                break;
            },
            2 => {
                println!("Fahrenheit to Celsius Converter");
                break;
            },
            3 => {
                println!("Project 3");
                break;
            },
            4 => {
                println!("Project 4");
                break;
            },
            _ => println!("Invalid option"),
        }
        println!("Selected option: {option}");

    }
}

fn celsius_to_fahrenheit() {
    println!("Enter a temperature in Celsius.");

    let mut temp_in_c = String::new();

    io::stdin().read_line(&mut temp_in_c).expect("Failed to read temperature");

    let temp_in_c: f64 = temp_in_c.trim().parse().expect("Not a valid number");

    let temp_in_f = temp_in_c * 1.8 + 32.0;

    println!("{temp_in_c}°C in Fahrenheit is: {temp_in_f}°F.");
}
