use std::io;

fn main() {
    println!("Welcome to the Temperature Converter!");
    let new_sign: char;
    let original_sign: char;

    let temperature: f32 = loop {
        println!("Please enter the temperature that you wish to convert");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Unable to read line");

        match choice.trim().parse() {
            Ok(choice) => break choice,
            Err(_) => {
                println!("Please enter a float.");
                continue;
            }
        };
    };

    let converted_temp: f32 = loop {
        let conversion_type: i32 = loop {
            prompt_for_choice();
            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Unable to read line");

            match choice.trim().parse() {
                Ok(choice) => break choice,
                Err(_) => {
                    println!("Please enter a number.");
                    continue;
                }
            };
        };

        if conversion_type == 2 {
            new_sign = 'C';
            original_sign = 'F';
            break farenheit_to_celcius(temperature);
        } else if conversion_type == 1 {
            new_sign = 'F';
            original_sign = 'C';
            break celcius_to_farenheit(temperature);
        } else {
            println!("Please enter either 1 or 2");
            continue;
        }
    };

    println!("{temperature}{original_sign} is {converted_temp}{new_sign}")
}

fn prompt_for_choice() {
    println!("Do you wish to convert from C to F or F to C (1 or 2)?");
    println!("1. C -> F");
    println!("2. F -> C");
}

fn celcius_to_farenheit(c: f32) -> f32 {
    (c * 1.8) + 32.0
}

fn farenheit_to_celcius(f: f32) -> f32 {
    (f - 32.0) / 1.8
}
