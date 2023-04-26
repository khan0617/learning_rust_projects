use std::io;

fn fahrenheight_to_celsius(f: f32) -> f32 {
    (f - 32.0) * (5.0/9.0)
}

fn celsius_to_fahrenheight(c: f32) -> f32 {
    (c * 1.8) + 32.0
}

pub fn conversion_game() {
    println!("Enter the unit you want to convert FROM (C or F): ");
    let mut choice = String::new();

    // prompt user to select unit to convert FROM
    while choice.trim().to_lowercase() != "c" && choice.trim().to_lowercase() != "f" {
        choice.clear();
        io::stdin().read_line(&mut choice).unwrap();
    }

    // determine the conversion function to call based on input
    let mut conversion_func: fn(f32) -> f32;
    let opposite_choice: &str;
    choice = choice.trim().to_lowercase();
    if choice == "f" {
        println!("Conversions will be from F to C...");
        conversion_func = fahrenheight_to_celsius;
        opposite_choice = "c";
    } else if choice == "c" {
        println!("Conversions will be from C to F...");
        conversion_func = celsius_to_fahrenheight;
        opposite_choice = "f";
    } else {
        panic!("Invalid input!!!");
    }


    // read numbers from user input and convert them accordingly
    println!("Enter a num in degrees ({}) to convert to ({}), or q to quit", 
        choice.to_uppercase(), opposite_choice.to_uppercase()); 

    let mut number_choice = String::new();
    loop {
        number_choice.clear();
        io::stdin()
            .read_line(&mut number_choice)
            .expect("Failed to read input!");

        if number_choice.trim() == "q" {
            break;
        }

        let number: f32 =  match number_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse input into float, maybe add a '.0' to the end?");
                continue;
            }
        };

        let converted_val = conversion_func(number);
        println!("{number} {} is {converted_val} {}", 
            choice.to_uppercase(), opposite_choice.to_uppercase());
    }
    println!("Goodbye...");
}