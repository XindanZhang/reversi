
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
