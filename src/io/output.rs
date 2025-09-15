use crate::{Player, Board};

pub fn winner_output(winner: Player, board: &Board){
    let points = winner.winner_points(board);
    match points {
        0 => println!("Draw!"),
        _ => println!("{} wins by {} points!", winner.winner(), points),
    }
}

pub fn print_invalid_move(){
    println!("Invalid move. Try again.");
}

pub fn print_no_valid_move(player: Player){
    println!("{} has no valid move.", player.as_char());
}