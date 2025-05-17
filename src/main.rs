use guessing_game::GuessingGame;
use std::io;

fn main() -> io::Result<()> {
    let mut game = GuessingGame::new();
    game.run()
}
