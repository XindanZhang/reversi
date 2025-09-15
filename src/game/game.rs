use crate::{Board, Player, Coordinates};
use crate::game::board::{create_board, display_board};

pub struct Game {
    board: Board,
    player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            // initial board configuration
            board: create_board(),
            // lets black disk go first
            player: Player::Black,
        }
    }

    pub fn run(&self) {
        // displays the initial board
        display_board(&self.board);
        loop{
            // TODO: Implement the loop.
            let input = get_input(self.player);
        }


    }
}
