use std::io;

fn main() {
    println!("\nWelcome to my compilation of mini Rust projects!\n\
    This program has been written by @aleksandrkaron, from the knowledge I gained from the Rust book.");

    loop {
        println!("\n>>> Select a program to run. <<<\n\n\
        1) Temperature Converter\n\
        2) Project 2\n\
        3) Project 3\n\n\
        Please enter your input (1-3).");

        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("INVALID OPTION!");
                continue;
            },
        };

        match option {
            1 => {
                temp_converter();
                break;
            },
            2 => {
                println!("Project 2");
                break;
            },
            3 => {
                println!("Project 3");
                break;
            },
            _ => println!("\nINVALID OPTION!"),
        }

    }
}

fn temp_converter() {
    print!("{}[2J", 27 as char);
    println!("\nWelcome to the Temperature Converter!\n");
    loop {
        println!("\n>>> Which temperature unit would you like to convert? <<<\n\n\
        1) Celsius\n\
        2) Fahrenheit\n\n\
        Please enter your input (1-2).");

        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("INVALID OPTION!");
                continue;
            },
        };

        match option {
            1 => {
                celsius_to_fahrenheit();
                break;
            },
            2 => {
                fahrenheit_to_celsius();
                break;
            },
            _ => println!("\nINVALID OPTION!"),
        }
    }
}
fn celsius_to_fahrenheit() {
    println!("\nYou chose to convert from Celsius to Fahrenheit.");

    loop {
        println!("\n >>> Enter a temperature in Celsius. <<<");

        let mut temp_in_c = String::new();

        io::stdin().read_line(&mut temp_in_c).expect("Failed to read temperature");

        let temp_in_c: f64 = match temp_in_c.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("INVALID OPTION!");
                continue;
            },
        };

        let temp_in_f = (temp_in_c * (9.0 / 5.0)) + 32.0;

        println!("\n{}°C in Fahrenheit is: {:.2}°F.", temp_in_c, temp_in_f);

        break;
    }
}

fn fahrenheit_to_celsius() {
    println!("\nYou chose to convert from Fahrenheit to Celsius.");

    loop {
        println!("\n >>> Enter a temperature in Fahrenheit. <<<");

        let mut temp_in_f = String::new();

        io::stdin().read_line(&mut temp_in_f).expect("Failed to read temperature");

        let temp_in_f: f64 = match temp_in_f.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nINVALID OPTION!");
                continue;
            },
        };

        let temp_in_c = (temp_in_f - 32.0) * (5.0 / 9.0);

        println!("\n{}°F in Celsius is: {:.2}°C.", temp_in_f, temp_in_c);

        break;
    }
}
