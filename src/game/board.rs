use crate::{Board, Player};

pub fn create_board() -> Board {
    let mut board: Board = [[None; 8]; 8];

    /*
        abcdefgh
    a ........
    b ........
    c ........
    d ...WB...
    e ...BW...
    f ........
    g ........
    h ........
    */
    board[3][3] = Some(Player::White);
    board[3][4] = Some(Player::Black);
    board[4][3] = Some(Player::Black);
    board[4][4] = Some(Player::White);

    board
}


pub fn display_board(board: &Board) {
    println!("  abcdefgh");
}

