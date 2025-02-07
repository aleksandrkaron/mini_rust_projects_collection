use std::io;
use ordinal::Ordinal;

fn main() {
    print!("{}[2J", 27 as char);
    println!("\nWelcome to my compilation of mini Rust projects!\n\
    This program has been written by @aleksandrkaron, from the knowledge I gained from the Rust book.");

    loop {
        println!("\n>>> Select a program to run. <<<\n\n\
        1) Temperature Converter\n\
        2) Fibonacci Sequence Generator\n\
        3) Christmas Carol Generator\n\n\
        Please enter your input (1-3).");

        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("{}[2J", 27 as char);
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
                fibonacci_sequence();
                break;
            },
            3 => {
                christmas_carol();
                break;
            },
            _ => {
                print!("{}[2J", 27 as char);
                println!("\nINVALID OPTION!");
            },
        }

    }
}

fn temp_converter() {
    print!("{}[2J", 27 as char);
    println!("\nWelcome to the Temperature Converter!\n");
    loop {
        println!(">>> Which temperature unit would you like to convert? <<<\n\n\
        1) Celsius\n\
        2) Fahrenheit\n\n\
        Please enter your input (1-2).");

        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("{}[2J", 27 as char);
                println!("INVALID OPTION!\n");
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
            _ => {
                print!("{}[2J", 27 as char);
                println!("\nINVALID OPTION!");
            },
        }
    }
}
fn celsius_to_fahrenheit() {
    print!("{}[2J", 27 as char);
    println!("\nYou chose to convert from Celsius to Fahrenheit.");

    loop {
        println!("\n>>> Enter a temperature in Celsius. <<<");

        let mut temp_in_c = String::new();

        io::stdin().read_line(&mut temp_in_c).expect("Failed to read temperature");

        let temp_in_c: f64 = match temp_in_c.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("{}[2J", 27 as char);
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
    print!("{}[2J", 27 as char);
    println!("\nYou chose to convert from Fahrenheit to Celsius.");

    loop {
        println!("\n>>> Enter a temperature in Fahrenheit. <<<");

        let mut temp_in_f = String::new();

        io::stdin().read_line(&mut temp_in_f).expect("Failed to read temperature");

        let temp_in_f: f64 = match temp_in_f.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("{}[2J", 27 as char);
                println!("INVALID OPTION!");
                continue;
            },
        };

        let temp_in_c = (temp_in_f - 32.0) * (5.0 / 9.0);

        println!("\n{}°F in Celsius is: {:.2}°C.", temp_in_f, temp_in_c);

        break;
    }
}

fn fibonacci_sequence() {
    print!("{}[2J", 27 as char);
    println!("\nWelcome to the Fibonacci Sequence Generator!\n\
    This code uses the Binet's Formula to calculate the n-th Fibonacci number!\n");

    loop {
        println!(">>> Please enter the n-th Fibonacci number you’d like to calculate. <<< \n");

        let mut number = String::new();

        io::stdin().read_line(&mut number).expect("Failed to read line");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("{}[2J", 27 as char);
                println!("\nINVALID OPTION!\n");
                continue;
            },
        };

        let number_as_float: f64 = number as f64;

        // 1 divided by the square root of 5
        let sequence_1: f64 = 1.0 / 5_f64.sqrt();
        // 1 plus the square root of 5, divided by 2, to the power of the n-th number
        let sequence_2: f64 = ((1.0 + 5_f64.sqrt()) / 2.0).powf(number_as_float);
        // 1 minus the square root of 5, divided by 2, to the power of the n-th number
        let sequence_3: f64 = ((1.0 - 5_f64.sqrt()) / 2.0).powf(number_as_float);

        let calculation: f64 = sequence_1 * (sequence_2 - sequence_3);

        println!("\nThe {} number of the Fibonacci Sequence is: {:.0}.", Ordinal(number), calculation);
        break;
    }
}

fn christmas_carol() {
    let ordered_items =
        [("first", "And a partridge in a pear tree."),
            ("second", "Two turtle doves,"),
            ("third", "Three french hens,"),
            ("fourth", "Four calling birds,"),
            ("fifth", "Five golden rings,"),
            ("sixth", "Six geese a-laying,"),
            ("seventh", "Seven swans a-swimming,"),
            ("eighth", "Eight maids a-milking,"),
            ("ninth", "Nine ladies dancing,"),
            ("tenth", "Ten lords a-leaping,"),
            ("eleventh", "Eleven pipers piping,"),
            ("twelfth", "Twelve drummers drumming,"),];

    print!("{}[2J", 27 as char);
    println!("\nWelcome to the Christmas Carol Generator!\n");
    println!("This version of The Twelve Days of Christmas has been generated using X, \n\
    to automatically generate the repeated parts.\n\n");
    println!("The 12 Days of Christmas\n\n\
    ----\n");

    println!("On the first day of Christmas,\n\
    my true love sent to me,\n\
    A partridge in a pear tree\n");

    let mut increasing_index = 1;

    // Prints each paragraph in increasing numbers
    while increasing_index < ordered_items.len() && increasing_index >= 0 {
        println!("On the {} day of Christmas,", ordered_items[increasing_index].0);
        println!("my true love sent to me");

        let mut decreasing_index = increasing_index;
        // Prints each line from the current paragraph in decreasing numbers
        while decreasing_index > 0 && decreasing_index < ordered_items.len() {
            println!("{}", ordered_items[decreasing_index].1);
            decreasing_index -= 1;
        }
        // Prints the remaining line at index 0
        println!("{}\n", ordered_items[decreasing_index].1);

        increasing_index += 1;

    }

    // On the first day of Christmas,
    // my true love gave to me
    // A partridge in a pear tree.

    //     On the second day of Christmas,
    // my true love gave to me
    // Two turtle doves,
    // And a partridge in a pear tree.

    //     On the third day of Christmas,
    // my true love gave to me
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree.

    //     On the fourth day of Christmas,
    // my true love gave to me
    // Four calling birds,
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree.
    //     On the fifth day of Christmas,
    // my true love gave to me
    // Five golden rings,
    // Four calling birds,
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree.
    //     On the sixth day of Christmas,
    // my true love gave to me
    // Six geese a-laying,
    // Five golden rings,
    // Four calling birds,
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree.
    //     On the seventh day of Christmas,
    // my true love gave to me
    // Seven swans a-swimming,
    // Six geese a-laying,
    // Five golden rings,
    // Four calling birds,
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree.
    //     On the eighth day of Christmas,
    // my true love gave to me
    // Eight maids a-milking,
    // Seven swans a-swimming,
    // Six geese a-laying,
    // Five golden rings,
    // Four calling birds,
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree.
    //     On the ninth day of Christmas,
    // my true love gave to me
    // Nine ladies dancing,
    // Eight maids a-milking,
    // Seven swans a-swimming,
    // Six geese a-laying,
    // Five golden rings,
    // Four calling birds,
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree.
    //     On the tenth day of Christmas,
    // my true love gave to me
    // Ten lords a-leaping,
    // Nine ladies dancing,
    // Eight maids a-milking,
    // Seven swans a-swimming,
    // Six geese a-laying,
    // Five golden rings,
    // Four calling birds,
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree.
    //     On the eleventh day of Christmas,
    // my true love gave to me
    // Eleven pipers piping,
    // Ten lords a-leaping,
    // Nine ladies dancing,
    // Eight maids a-milking,
    // Seven swans a-swimming,
    // Six geese a-laying,
    // Five golden rings,
    // Four calling birds,
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree.
    //     On the twelfth day of Christmas,
    // my true love gave to me
    // Twelve drummers drumming,
    // Eleven pipers piping,
    // Ten lords a-leaping,
    // Nine ladies dancing,
    // Eight maids a-milking,
    // Seven swans a-swimming,
    // Six geese a-laying,
    // Five golden rings,
    // Four calling birds,
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree!


}