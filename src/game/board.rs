use crate::{Board, Player};

// Creates the board in the initial state.
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

// Displays the board in the required format.
pub fn display_board(board: &Board) {
    println!("  abcdefgh");
    for (i, row) in board.iter().enumerate() {
        // prints the first column letter
        print!("{}", char::from(b'a' + i as u8));
        for cell in row {
            // prints the specific row cell
            // fn display_cell() is in lib.rs
            print!("{}", Player::display_cell(cell));
        }
        println!();
    }


}

