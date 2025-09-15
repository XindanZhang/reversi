use std::io::{self, Write};
use crate::{Player, InputError};

pub fn get_input(player: Player) -> Result<(u8, u8), InputError> {
    // gets the input from the user
    print!("Enter move for colour {} (RowCol): ", player.as_char());
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
    let chars: Vec<char> = trimmed_input.to_lowercase().chars().collect();
    let row_char = chars[0];
    let col_char = chars[1];

    // validates characters are in valid range before converting
    if row_char < 'a' || row_char > 'h' || col_char < 'a' || col_char > 'h' {
        return Err(InputError::InvalidFormat);
    }

    // converts the chars to numbers
    let row = row_char as u8 - b'a';
    let col = col_char as u8 - b'a';

    Ok((row, col))
}   


