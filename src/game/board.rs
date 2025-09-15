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

// Counts the number of black and white pieces on the board.
pub fn cnt_black_white(board: &Board) -> (u8, u8) {
    let mut black_cnt = 0;
    let mut white_cnt = 0;
    for row in board{
        for &cell in row{
            match cell{
                Some(Player::Black) => black_cnt +=1,
                Some(Player::White) => white_cnt +=1,
                None => {}
            }
        }
    }
    (black_cnt, white_cnt)
}

// Displays the board in the required format.
pub fn display_board(board: &Board) {
    println!("  abcdefgh");
    for (i, row) in board.iter().enumerate() {
        // prints the first column letter
        print!("{}", char::from(b'a' + i as u8));
        // prints the specific row cell
        for &cell in row {
            // fn display_cell() is in lib.rs
            print!("{}", Player::display_cell(cell));
        }
        println!();
    }
}
