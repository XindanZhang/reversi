use crate::{Coordinates, InputError, Player};
use std::io::{self, Write};

pub fn print_prompt_text(player: Player) {
    print!("Enter move for colour {} (RowCol): ", player.as_char());
    io::stdout().flush().expect("Failed to flush stdout.");
}

pub fn get_input() -> Result<Coordinates, InputError> {
    // reads the input from the user
    let mut coordinates = String::new();
    io::stdin()
        .read_line(&mut coordinates)
        .expect("Failed to read line.");

    // removes the whitespace
    let trimmed_input = coordinates.trim();

    // checks if there are two characters in the input
    if trimmed_input.len() != 2 {
        return Err(InputError::InputInvalidFormat);
    }

    // gets the row and column from the input
    let chars: Vec<char> = trimmed_input.to_lowercase().chars().collect();
    let row_char = chars[0];
    let col_char = chars[1];

    // validates characters are in valid range before converting
    if row_char < 'a' || row_char > 'h' || col_char < 'a' || col_char > 'h' {
        return Err(InputError::InputOutOfBounds);
    }

    // converts the chars to numbers
    let row = row_char as u8 - b'a';
    let col = col_char as u8 - b'a';

    // creates the coordinates
    let coordinates = Coordinates::new(row, col);
    // returns the coordinates
    Ok(coordinates)
}
