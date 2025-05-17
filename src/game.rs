use crate::constants::*;
use crate::utils::MessageBuilder;
use colored::Colorize;
use rand::prelude::*;
use rand::rng;
use std::cmp::Ordering;
use std::io::{self, Write};

// Type aliases
pub type Guess = u32;

/// Game state enum
#[derive(Debug)]
enum GameState {
    Playing,
    Won,
    Lost,
}

/// Represents the state of the guessing game
pub struct GuessingGame {
    secret_number: Guess,
    guesses_remaining: u32,
    state: GameState,
}

impl GuessingGame {
    /// Creates a new game instance with a random secret number
    pub fn new() -> Self {
        Self {
            secret_number: rng().random_range(MIN_NUMBER..=MAX_NUMBER),
            guesses_remaining: MAX_GUESSES,
            state: GameState::Playing,
        }
    }

    /// Runs the main game loop
    pub fn run(&mut self) -> io::Result<()> {
        self.print_welcome();

        while self.is_playing() {
            self.print_remaining_guesses();
            let guess = self.get_user_guess()?;
            self.process_guess(guess);
        }

        self.print_game_over();
        Ok(())
    }

    /// Checks if the game is still in progress
    fn is_playing(&self) -> bool {
        matches!(self.state, GameState::Playing) && self.guesses_remaining > 0
    }

    /// Gets and validates the user's guess
    fn get_user_guess(&self) -> io::Result<Guess> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<Guess>() {
            Ok(num) if (MIN_NUMBER..=MAX_NUMBER).contains(&num) => Ok(num),
            Ok(_) => {
                self.print_range_error();
                self.get_user_guess()
            }
            Err(_) => {
                MessageBuilder::new()
                    .add("Please enter a valid number!", |s| s.red().bold())
                    .print();
                self.get_user_guess()
            }
        }
    }

    /// Processes the user's guess and provides feedback
    fn process_guess(&mut self, guess: Guess) {
        match guess.cmp(&self.secret_number) {
            Ordering::Equal => self.state = GameState::Won,
            ordering => {
                self.print_hint(ordering);
                self.guesses_remaining -= 1;
                if self.guesses_remaining == 0 {
                    self.state = GameState::Lost;
                }
            }
        }
    }

    // Helper methods for printing
    fn print_welcome(&self) {
        println!(); // Keep the newline
        MessageBuilder::new()
            .add(WELCOME_MSG, |s| s.bold().cyan())
            .print();

        MessageBuilder::new().add(BORDER, |s| s.cyan()).print();

        MessageBuilder::new()
            .add(
                RANGE_MSG
                    .replace("{min}", &MIN_NUMBER.to_string())
                    .replace("{max}", &MAX_NUMBER.to_string()),
                |s| s.italic(),
            )
            .print();

        MessageBuilder::new()
            .add(GUESSES_MSG.replace("{}", &MAX_GUESSES.to_string()), |s| {
                s.bold().green()
            })
            .print();

        MessageBuilder::new().add(BORDER, |s| s.cyan()).print();
    }

    fn print_remaining_guesses(&self) {
        println!(); // Keep the newline
        MessageBuilder::new()
            .add(
                GUESSES_REMAINING.replace("{}", &self.guesses_remaining.to_string()),
                |s| s.bold().yellow(),
            )
            .print();

        MessageBuilder::new()
            .add(GUESS_PROMPT, |s| s.green())
            .print();
        io::stdout().flush().unwrap();
    }

    fn print_range_error(&self) {
        MessageBuilder::new()
            .add("Please enter a number between", |s| s.red())
            .add(MIN_NUMBER, |s| s.yellow())
            .add("and", |s| s.red())
            .add(MAX_NUMBER, |s| s.yellow())
            .print();
    }

    fn print_hint(&self, ordering: Ordering) {
        let (hint, emoji) = match ordering {
            Ordering::Less => ("Too small", "⬇️"),
            Ordering::Greater => ("Too big", "⬆️"),
            Ordering::Equal => unreachable!(),
        };
        MessageBuilder::new()
            .add(emoji, |s| s.normal())
            .add(hint, |s| s.red().bold())
            .print();
    }

    fn print_game_over(&self) {
        println!(); // Keep the newline
        MessageBuilder::new().add(BORDER, |s| s.cyan()).print();

        match self.state {
            GameState::Won => {
                MessageBuilder::new()
                    .add(WIN_MSG, |s| s.bold().green())
                    .print();
                MessageBuilder::new()
                    .add(
                        FOUND_NUMBER.replace("{}", &self.secret_number.to_string()),
                        |s| s.green(),
                    )
                    .print();
            }
            GameState::Lost => {
                MessageBuilder::new()
                    .add(LOSE_MSG, |s| s.bold().red())
                    .print();
                MessageBuilder::new()
                    .add(
                        WAS_NUMBER.replace("{}", &self.secret_number.to_string()),
                        |s| s.red(),
                    )
                    .print();
            }
            GameState::Playing => unreachable!(),
        }

        MessageBuilder::new().add(BORDER, |s| s.cyan()).print();
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from parent module

    #[test]
    fn test_new_game() {
        let game = GuessingGame::new();
        assert_eq!(game.guesses_remaining, MAX_GUESSES);
        assert!(matches!(game.state, GameState::Playing));
        assert!(game.secret_number >= MIN_NUMBER);
        assert!(game.secret_number <= MAX_NUMBER);
    }

    #[test]
    fn test_game_state_transitions() {
        let mut game = GuessingGame::new();
        game.secret_number = 50; // Set a known secret number for testing

        // Test winning
        game.process_guess(50);
        assert!(matches!(game.state, GameState::Won));

        // Test losing
        let mut game = GuessingGame::new();
        game.secret_number = 50;
        game.guesses_remaining = 1;
        game.process_guess(30); // Wrong guess
        assert!(matches!(game.state, GameState::Lost));
    }

    #[test]
    fn test_is_playing() {
        let mut game = GuessingGame::new();
        assert!(game.is_playing());

        // Test when won
        game.state = GameState::Won;
        assert!(!game.is_playing());

        // Test when lost
        game.state = GameState::Lost;
        assert!(!game.is_playing());

        // Test when out of guesses
        game.state = GameState::Playing;
        game.guesses_remaining = 0;
        assert!(!game.is_playing());
    }
}
