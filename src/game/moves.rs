use crate::{Board, Coordinates, Player};

pub enum MoveError {
    MoveOutOfBounds,
    MoveOccupied,
}

pub enum CellState {
    Occupied,
    Empty,
}

pub fn flip_disks(board: &mut Board, player: &Player, coordinates: &Coordinates) -> Board {
    // TODO: Implement the flip disks function.
    let mut new_board = board.clone();
    //
    new_board[coordinates.row][coordinates.col] = match player {
        Player::Black => {
            for row in 0..coordinates.row {

        },
        Player::White => Some(Player::Black),
    };
    new_board
}

// already checked if the input is in bounds
// now checks if the move is occupied
pub fn is_valid_move(board: &Board, player: &Player, coordinates: &Coordinates) -> Result<bool, MoveError> {

    match board[coordinates.row][coordinates.col] {
        Some(player) => {
            return Err(MoveError::MoveOccupied);
        }
        None => {
            return Ok(true);
        }
    }
}

// flip disks
pub fn make_move(board: &mut Board, player: &Player, coordinates: &Coordinates) -> bool {
    // TODO: Implement the make move function.
    flip_disks();

    true
}

// Get the cell state and coordinates.
pub fn get_cell_state(board: &Board) -> (CellState, Coordinates) {
    for row in 0..8 {
        for col in 0..8 {
            match board[row][col] {
                Some(_) => {
                    let (cell_state, coordinates) = (CellState::Occupied, (row, col));
                    return (cell_state, coordinates);
                }
                None => {
                    return (CellState::Empty, (row, col));
                }
            }
        }
    }
}

pub fn get_valid_coordinates(cell_state: &CellState, coordinates: &Coordinates) -> Vec<Coordinates> {
    let mut valid_coordinates = vec![];
    let vacant_coordinates = vec![
        (coordinates.row + 0, coordinates.col + 1),
        (coordinates.row + 1, coordinates.col + 0),
        (coordinates.row + 0, coordinates.col - 1),
        (coordinates.row - 1, coordinates.col + 0),
        (coordinates.row + 1, coordinates.col + 1),
        (coordinates.row - 1, coordinates.col - 1),
        (coordinates.row + 1, coordinates.col - 1),
        (coordinates.row - 1, coordinates.col + 1),
    ];
    for coordinate in vacant_coordinates {
        match cell_state {
            CellState::Empty => {
                valid_coordinates.push(coordinate);
            }
            CellState::Occupied => {
                continue;
            }
        }
    }
    valid_coordinates
}

pub fn has_valid_move(board: &Board, player: &Player) -> Result<bool, MoveError> {
    // gets the cell state and coordinates
    let ( cell_state, coordinates ) = get_cell_state(board);
    // gets the valid coordinates
    let valid_coordinates = get_valid_coordinates(&cell_state, &coordinates);

    if valid_coordinates.is_empty() {
        return Err(MoveError::MoveOutOfBounds);
    }

    return Ok(true);
}
