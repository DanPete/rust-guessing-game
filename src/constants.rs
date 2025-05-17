// src/constants.rs

// Game settings
pub const MIN_NUMBER: u32 = 1;
pub const MAX_NUMBER: u32 = 100;
pub const MAX_GUESSES: u32 = 7;

// Message constants
pub const BORDER: &str = "----------------------------------------";
pub const WELCOME_MSG: &str = "Welcome to the Number Guessing Game!";
pub const RANGE_MSG: &str = "I'm thinking of a number between {min} and {max}";
pub const RANGE_ERROR_MSG: &str = "Please enter a number between {min} and {max}";
pub const GUESSES_MSG: &str = "You have {} guesses to find it!";
pub const GUESS_PROMPT: &str = "Your guess: ";
pub const GUESSES_REMAINING: &str = "Guesses remaining: {}";
pub const WIN_MSG: &str = "🎉 CONGRATULATIONS! 🎉";
pub const LOSE_MSG: &str = "😢 GAME OVER 😢";
pub const FOUND_NUMBER: &str = "You found the number: {}";
pub const WAS_NUMBER: &str = "The number was: {}";
