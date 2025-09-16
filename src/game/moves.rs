use crate::{Board, Coordinates, Player};


pub fn flip_disks(board: &mut Board, player: &Player, coordinates: &Coordinates) -> Board {
    // TODO: Implement the flip disks function.
}

pub fn is_valid_move(board: &Board, player: &Player, coordinates: &Coordinates) -> Result<bool, MoveError> {
    if board[coordinates.row][coordinates.col].is_some() {
        return false;
    }
    
    false
}

// flip disks
pub fn make_move(board: &mut Board, player: &Player, coordinates: &Coordinates) -> bool {
    // TODO: Implement the make move function.
    flip_disks();

    true
}


// check if the player has any valid moves
pub fn has_valid_move(board: &Board, player: &Player) -> bool {
    for row in 0..8 {
        for col in 0..8 {
            if board[row][col].is_none() {
                let coordinates = Coordinates::new(row as u8, col as u8);
                if is_valid_move(board, player, &coordinates) {
                    return true;
                }
            }
        }
    }
    false 
}

