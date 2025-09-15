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
    for (i, row) in board.iter().enumerate() {
        // prints the first column letter
        print!("{}", char::from_u8(b'a' + i as u8).unwrap());
        for cell in row {
            // prints the specific row cell
            // fn display_cell() is in lib.rs
            print!("{}", Player::display_cell(cell));
        }
    }


}

