use crate::{Board, Coordinates, Player};

fn all_directions() -> [(i8, i8); 8] {
    // 8 directions around the center piece
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
}

/// There are two cases for an invalid move:
/// If the position is not empty, return false
/// If the position doesn't exist in the 8 possible positions, return false
pub fn is_valid_move(board: &Board, player: &Player, coordinates: &Coordinates) -> bool {
    // if the position is not empty, return false
    if board[coordinates.row as usize][coordinates.col as usize].is_some() {
        return false;
    }

    for direction in all_directions().iter() {
        // current row adds one of the directions
        let mut row = coordinates.row as i8 + direction.0;
        let mut col = coordinates.col as i8 + direction.1;

        // if the position is out of bounds, return false
        if row < 0 || row >= 8 || col < 0 || col >= 8 {
            break;
        }

        // if the position is occupied, checks if the player can flip the disk in that direction
        if board[row as usize][col as usize].is_some() {
            // search if the player can flip the disk till the same color is found
            if can_flip(board, player, coordinates, direction) {
                return true;
            }
            break;
        }
    }
    false
}

/// Check if the player can flip the disk in the given direction.
fn can_flip(board: &Board, player: &Player, coordinates: &Coordinates, direction: &(i8, i8)) -> bool {
    let mut row = coordinates.row as i8 + direction.0;
    let mut col = coordinates.col as i8 + direction.1;
    let player2 = player.next();
    let mut flag = false;

    // while the position is in bounds
    while row < 8 && col < 8 && row >= 0 && col >= 0 {
        match board[row as usize][col as usize] {
            Some(pos) if pos == player2 => {
                flag = true;
            }
            Some(pos) if pos == *player => {
                return flag;
            }
            _ => return false,
        }
        // move to the next position
        row += direction.0;
        col += direction.1;
    }
    false
}

/// Get the vector of positions to flip in a specific direction.
fn positions_to_flip(board: &Board, player: &Player, coordinates: &Coordinates, direction: &(i8, i8)) -> Vec<Coordinates> {
    let mut positions = Vec::new();
    let mut row = coordinates.row as i8 + direction.0;
    let mut col = coordinates.col as i8 + direction.1;

    while row < 8 && col < 8 && row >= 0 && col >= 0 {
        match board[row as usize][col as usize] {
            Some(pos) if pos == player2 => {
                positions.push(Coordinates::new(row as u8, col as u8));
            }
            Some(pos) if pos == *player => {
                return positions;
            }
            _ => return vec![],
        }
        row += direction.0;
        col += direction.1;
    }
    positions
}

/// Flip the pieces in all valid directions.
pub fn flip_pieces(board: &mut Board, player: &Player, coordinates: &Coordinates) {
    // places the new piece on the board
    board[coordinates.row as usize][coordinates.col as usize] = Some(*player);

    // gets all the vector of positions to flip
    let positions = positions_to_flip(board, player, coordinates, direction);
    for pos in positions {
        // flips the pieces on the board
        board[pos.row as usize][pos.col as usize] = Some(*player);
    }
}

pub fn make_move(board: &mut Board, player: &Player, coordinates: &Coordinates) -> bool {
    if is_valid_move(board, player, coordinates) {
        // implements the flip pieces.
        flip_pieces(board, player, coordinates);
        return true;
    }
    false
}

pub fn has_valid_move(board: &Board, player: &Player) -> bool {
    for row in 0..8 {
        for col in 0..8 {
            // checks all the empty positions on the current board
            if board[row][col].is_none() {
                let coordinates = Coordinates::new(row as u8, col as u8);
                // checks if any of the empty positions are valid moves
                // if one is valid, the player has a valid move
                // returns true
                if is_valid_move(board, player, &coordinates) {
                    return true;
                }
            }
        }
    }
    false
}
