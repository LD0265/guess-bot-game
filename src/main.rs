mod guesser;

use guesser::Guesser;
use std::io::Write;
use std::{io, process};

pub fn wait_for_exit() {
    println!("\nPress enter to exit program...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
    process::exit(1);
}

fn main() {
    let mut guesser: Guesser = Guesser::new(0..=0, false);
    
    let mut range_input = String::new();
    let mut mode_input = String::new();
    
    print!("Define an inclusive range for guesser (ex. 0, 100): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut range_input).expect("Failed to read line");

    let range = range_input.split(',').collect::<Vec<&str>>();
    
    if range.len() != 2 || !range_input.contains(",") {
        println!("Invalid input format");
        wait_for_exit();
    }
    
    let min_result = range[0].trim().parse::<i32>();
    let max_result = range[1].trim().parse::<i32>();
    
    let mut predicted_tries: u8 = 0;
    
    match (min_result, max_result) {
        (Ok(min), Ok(max)) => {
            print!("Auto or Manual mode: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut mode_input).expect("Failed to read line");

            let first_char = mode_input.chars().next().unwrap().to_ascii_lowercase();
            let auto_mode = if first_char == 'a' { true } else if first_char == 'm' { false } else {
                println!("Invalid mode");
                wait_for_exit();
                false
            };
            
            // this is not epic :(
            predicted_tries = (3.0 * (min.abs() as f64 + max.abs() as f64).log(10.0)) as u8 + 1;
            
            guesser = Guesser::new(min..=max, auto_mode);
        },
        
        (Err(_), _) => {
            println!("Error: The first part '{}' is not a valid integer.", range[0]);
            wait_for_exit();
        },
        
        (_, Err(_)) => {
            println!("Error: The second part '{}' is not a valid integer.", range[1]);
            wait_for_exit();
        }
    }
    
    println!();
    println!("I predict I can guess the number in {} tries", predicted_tries);
    println!("My first guess is {}", guesser.make_guess());
    
    loop {
        guesser.handle_response();
        println!();
        println!("My guess is {}", guesser.make_guess());
    }
}