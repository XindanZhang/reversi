mod game;
mod io;

use reversi::{Game, Board, Player};

fn main() {
    let game = Game::new();
    game.run();
}