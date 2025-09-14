pub mod game;
pub mod io;

pub use game::game::Game;


// black and white are the players
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    // shows the player as a character
    pub fn as_char(self) -> char {
        match self {
            Player::Black => 'B',
            Player::White => 'W',
        }
    }

    // chooses the next player to play
    pub fn next(self) -> Self{
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    // shows the winner of the game
    pub fn winner(self) -> String {
        match self {
            Player::Black => "Black".to_string(),
            Player::White => "White".to_string(),
        }
    }

}

pub struct Coordinates {
    pub row: u8,
    pub column: u8,
}

impl Coordinates {
    pub fn new(row: u8, column: u8) -> Self {
        Self { row, column }
    }
}
// the board of the game
pub struct Board {
}