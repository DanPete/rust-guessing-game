use colored::Colorize;
use rand::prelude::*;
use rand::rng;
use std::cmp::Ordering;
use std::io::{self, Write};

// Constants
const MIN_NUMBER: u32 = 1;
const MAX_NUMBER: u32 = 100;
const MAX_GUESSES: u32 = 7;

// Type alias for better readability
pub type Guess = u32;

/// Represents the state of the guessing game
pub struct GuessingGame {
    secret_number: Guess,
    guesses_remaining: u32,
}

impl GuessingGame {
    /// Creates a new game instance with a random secret number
    pub fn new() -> Self {
        Self {
            secret_number: rng().random_range(MIN_NUMBER..=MAX_NUMBER),
            guesses_remaining: MAX_GUESSES,
        }
    }

    /// Runs the main game loop
    pub fn run(&mut self) -> io::Result<()> {
        self.print_welcome();

        while self.guesses_remaining > 0 {
            self.print_remaining_guesses();
            let guess = self.get_user_guess()?;

            if self.process_guess(guess) {
                return Ok(());
            }
            self.guesses_remaining -= 1;
        }

        self.print_game_over(false);
        Ok(())
    }

    /// Gets and validates the user's guess
    fn get_user_guess(&self) -> io::Result<Guess> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<Guess>() {
            Ok(num) if (MIN_NUMBER..=MAX_NUMBER).contains(&num) => Ok(num),
            Ok(_) => {
                println!("{}", "Please enter a number between".red());
                println!(
                    "{} and {}",
                    MIN_NUMBER.to_string().yellow(),
                    MAX_NUMBER.to_string().yellow()
                );
                self.get_user_guess()
            }
            Err(_) => {
                println!("{}", "Please enter a valid number!".red().bold());
                self.get_user_guess()
            }
        }
    }

    /// Processes the user's guess and provides feedback
    /// Returns true if the game should end (win), false otherwise
    fn process_guess(&self, guess: Guess) -> bool {
        match guess.cmp(&self.secret_number) {
            Ordering::Equal => {
                self.print_game_over(true);
                true
            }
            ordering => {
                self.print_hint(ordering);
                false
            }
        }
    }

    // Helper methods for printing
    fn print_welcome(&self) {
        println!("\n{}", "Welcome to the Number Guessing Game!".bold().cyan());
        println!("{}", "----------------------------------------".cyan());
        println!(
            "{}",
            format!(
                "I'm thinking of a number between {} and {}",
                MIN_NUMBER.to_string().yellow(),
                MAX_NUMBER.to_string().yellow()
            )
            .italic()
        );
        println!(
            "{}",
            format!(
                "You have {} guesses to find it!",
                MAX_GUESSES.to_string().bold().green()
            )
        );
        println!("{}", "----------------------------------------".cyan());
    }

    fn print_remaining_guesses(&self) {
        println!(
            "\n{}",
            format!(
                "Guesses remaining: {}",
                self.guesses_remaining.to_string().bold().yellow()
            )
        );
        print!("{}", "Your guess: ".green());
        io::stdout().flush().unwrap(); // Make sure prompt appears before input
    }

    fn print_hint(&self, ordering: Ordering) {
        let (hint, emoji) = match ordering {
            Ordering::Less => ("Too small", "⬇️"),
            Ordering::Greater => ("Too big", "⬆️"),
            Ordering::Equal => unreachable!(),
        };
        println!("{} {}", emoji, hint.red().bold());
    }

    fn print_game_over(&self, won: bool) {
        println!("\n{}", "----------------------------------------".cyan());
        if won {
            println!("{}", "🎉 CONGRATULATIONS! 🎉".bold().green());
            println!(
                "{}",
                format!(
                    "You found the number: {}",
                    self.secret_number.to_string().bold().yellow()
                )
                .green()
            );
        } else {
            println!("{}", " GAME OVER 😢".bold().red());
            println!(
                "{}",
                format!(
                    "The number was: {}",
                    self.secret_number.to_string().bold().yellow()
                )
                .red()
            );
        }
        println!("{}", "----------------------------------------".cyan());

        if won {
            std::process::exit(0);
        }
    }
}
