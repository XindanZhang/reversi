pub mod game;
pub mod io;

pub use game::game::Game;
use crate::game::board::cnt_black_white;

use crate::io::output::{print_invalid_move, print_no_valid_move};


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

    // shows the winner points
    pub fn winner_points(self, board: &Board) -> u8 {
        let (black_cnt, white_cnt) = cnt_black_white(board);
        if black_cnt > white_cnt {
            black_cnt - white_cnt
        } else if black_cnt < white_cnt {
            white_cnt - black_cnt
        } else {
            0
        }
    }


    // shows the cell on the board
    pub fn display_cell(cell: Option<Player>) -> char {
        match cell {
            Some(player) => player.as_char(),
            None => '.',
        }
    }

}

pub struct Coordinates {
    pub row: u8,
    pub col: u8,
}

impl Coordinates {
    pub fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }

}

// the board of the game
pub type Board = [[Option<Player>; 8]; 8];
