mod game;
mod io;

use reversi::{Game, Board, Player, Coordinates, InputError};

fn main() {
    let mut game = Game::new();
    game.run();
}