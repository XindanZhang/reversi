use crate::{Player, Board};

pub fn winner_output(winner: Player, board: &Board){
    let points = winner.winner_points(board);
    match points {
        0 => println!("Draw!"),
        _ => println!("{} wins by {} points!", winner.as_char(), points),
    }
}