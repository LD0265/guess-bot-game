mod guesser;

use std::{io, process};
use std::io::Write;
use guesser::Guesser;

fn main() {
    let mut guesser: Guesser;
    
    let mut range_input = String::new();
    let mut mode_input = String::new();
    
    print!("Define an inclusive range for guesser (ex. 0, 100): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut range_input).expect("Failed to read line");

    let range = range_input.split(',').collect::<Vec<&str>>();
    
    print!("Auto or Manual mode: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut mode_input).expect("Failed to read line");
    
    let first_char = mode_input.chars().next().unwrap().to_ascii_lowercase();
    let auto_mode = if first_char == 'a' { true } else if first_char == 'm' { false } else {
        println!("Invalid mode");
        process::exit(1);
    };
    
    if range.len() != 2 || !range_input.contains(",") {
        println!("Invalid input format");
        process::exit(1);
    }
    
    let min_result = range[0].trim().parse::<i32>();
    let max_result = range[1].trim().parse::<i32>();
    
    match (min_result, max_result) {
        (Ok(min), Ok(max)) => {
            guesser = Guesser::new(min..=max, auto_mode);
        },
        
        (Err(_), _) => {
            println!("Error: The first part '{}' is not a valid integer.", range[0]);
            process::exit(1);
        },
        
        (_, Err(_)) => {
            println!("Error: The second part '{}' is not a valid integer.", range[1]);
            process::exit(1);
        }
    }

    println!("My guess is {}", guesser.make_guess());
    
    loop {
        guesser.handle_response();
        println!("My guess is {}", guesser.make_guess());
    }
}