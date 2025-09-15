use std::io::Write;
use std::io;
use crate::Player;

pub enum InputError {
    // When there is no valid move for the player.
    // prints "{} has no valid move."
    NoValidMove,
    // Invalid input from the user and retry is needed.
    // prints "Invalid move. Try again."
    InvalidMove,
}

pub fn get_input(player: Player) -> Option<(u8, u8)>{
    // gets the input from the user
    print!("Please enter move for colour {}: ", player.as_char());
    io::stdout().flush().expect("Failed to flush stdout.");

    // creates a new string to store the input
    let mut coordinates = String::new();
    // reads the input from the user
    io::stdin()
        .read_line(&mut coordinates)
        .expect("Failed to read line.");

    // gets the row and column from the input
    let row = coordinates.trim().chars().next().unwrap() as u8 - b'a';
    let col = coordinates.trim().chars().nth(1).unwrap() as u8 - b'1';

    // validates the input and returns the coordinates
    match validate_input(row, col, player) {
        Ok((row, col)) => Some((row, col)),
        Err(InputError::NoValidMove) => {
            println!("{} has no valid move.", player.as_char());
            None
        }
        Err(InputError::InvalidMove) => {
            println!("Invalid move. Try again.");
            game.run();
        }
    }
}

// Validates the input.
pub fn validate_input(row: u8, col: u8, player: Player) -> Result<(u8, u8), InputError> {
    // checks if the coordinates are valid
    if row < 8 && col < 8 {
        Ok((row, col))
    } else {
        Err(InputError::InvalidMove)
    }
}



