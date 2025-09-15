use crate::{Board, Player, Coordinates};
use crate::game::board::{create_board, display_board};
use crate::io::output::{winner_output, print_invalid_move, print_no_valid_move};

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
            // checks if the player has a valid move 
            // and if not, prints a message
            if !has_valid_move(&self.board, &self.player) {
                print_no_valid_move(self.player);
                // switches to the next player
                self.player = self.player.next();

                // checks if the next player has a valid move 
                // and if not
                if !has_valid_move(&self.board, &self.player) {
                    // prints the other player also has no valid move
                    print_no_valid_move(self.player);
                    // ends the game since both players have no valid move
                    break;
                }
                continue;
            }

            // if the player has a valid move, reads the input from the user
            match get_input(&self.board, self.player) {
                // if the input is valid
                Ok((row, col)) => {
                    if make_move(&mut self.board, &self.player, row, col) {
                        display_board(&self.board);
                    } else {
                        print_invalid_move();
                    }
                }
                Err() => {
                    print_invalid_move()(self.player);
                }
            }


            winner_output(self.player, &self.board);

        }


    }
}
