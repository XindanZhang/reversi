use crate::{Board, Player, Coordinates};
use crate::game::board::{create_board, display_board};

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: create_board(),
        }
    }

    pub fn run(&self) {
        // TODO: Implement the game.
    }
}
