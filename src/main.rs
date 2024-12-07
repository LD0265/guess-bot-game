mod guesser;

use std::{io, process};
use std::io::Write;
use guesser::Guesser;

fn main() {
    let mut guesser: Guesser;
    
    let mut input = String::new();
    
    print!("Define an inclusive range for guesser (ex. 0, 100): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let range = input.split(',').collect::<Vec<&str>>();
    
    if range.len() != 2 || !input.contains(",") {
        println!("Invalid input format");
        process::exit(1);
    }
    
    let min_result = range[0].trim().parse::<i32>();
    let max_result = range[1].trim().parse::<i32>();
    
    match (min_result, max_result) {
        (Ok(min), Ok(max)) => {
            guesser = Guesser::new(min..=max);
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