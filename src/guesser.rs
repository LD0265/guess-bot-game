use std::{io, process};
use std::io::Write;
use std::ops::RangeInclusive;

pub struct Guesser {
    guess_type: GuessType,
    left_value: i32,
    right_value: i32,
    prev_guess: i32,
    num_guesses: i32
}

pub enum GuessType {
    Low,
    High,
    First,
    Correct
}

impl Guesser {
    pub fn new(range: RangeInclusive<i32>) -> Guesser {
        let left_value = *range.start();
        let right_value = *range.end();

        Self {
            left_value,
            right_value,
            num_guesses: 0,
            prev_guess: 0,
            guess_type: GuessType::First
        }
    }

    pub fn set_guesstype(&mut self, guess_type: GuessType) {
        self.guess_type = guess_type;
    }

    pub fn make_guess(&mut self) -> i32 {
        // Use a left and a right, guess the middle value and adjust l and r accordingly
        
        let guess: i32;
        
        match self.guess_type {
            GuessType::Low => {
                self.right_value = self.prev_guess;
            },

            GuessType::High => {
                self.left_value = self.prev_guess;
            },

            GuessType::Correct => {
                println!("I guessed your number! Took {} tries", self.num_guesses);
                process::exit(0);
            },
            
            _ => {}
        }
        
        guess = Self::get_middle_value(self.left_value, self.right_value);
        self.prev_guess = guess;
        self.num_guesses += 1;
        
        guess
    }

    pub fn handle_response(&mut self) {
        loop {
            let mut user_response = String::new();

            print!("Am I higher, lower, or correct?: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut user_response).expect("Failed to read line");

            let first_char = user_response.chars().next().unwrap_or('?').to_ascii_lowercase();

            match first_char {
                'h' => {
                    self.set_guesstype(GuessType::High);
                    break;
                },
                'l' => {
                    self.set_guesstype(GuessType::Low);
                    break;
                },
                'c' => {
                    self.set_guesstype(GuessType::Correct);
                    break;
                },

                _ => {
                    println!("Invalid input");
                    continue;
                }
            }
        }
    }
    
    fn get_middle_value(left: i32, right: i32) -> i32 {
        (right + left) / 2
    }
}