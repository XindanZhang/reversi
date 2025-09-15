mod game;
mod io;

use reversi::{Game, Board, Player, Coordinates};

fn main() {
    let game = Game::new();
    game.run();
}