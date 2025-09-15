use std::io::{self, Write};
use crate::{Player, InputError};

pub fn get_input(player: Player) -> Result<(u8, u8), InputError> {
    // gets the input from the user
    print!("Please enter move for colour {}: ", player.as_char());
    io::stdout().flush().expect("Failed to flush stdout.");

    // reads the input from the user
    let mut coordinates = String::new();
    io::stdin()
        .read_line(&mut coordinates)
        .expect("Failed to read line.");

    // trims the input
    let trimmed_input = coordinates.trim();

    // checks if there are two characters in the input
    if trimmed_input.len() != 2 {
        return Err(InputError::InvalidFormat);
    }

    // gets the row and column from the input
    let mut chars = trimmed_input.chars();
    let row_char = chars.next().unwrap().to_lowercase();
    let col_char = chars.next().unwrap().to_lowercase();

    // converts the chars to numbers
    let row = row_char as u8 - b'a';
    let col = col_char as u8 - b'a';

    // checks if the row and column are out of bounds
    if row >= 8 || col >= 8 {
        return Err(InputError::InvalidFormat);
    }

    Ok((row, col))
}   


