use crate::{Board, Coordinates, Player};

pub fn flip_disks() {
    // TODO: Implement the flip disks function.
}

// Check if the input coordinates are valid for the current player to make a move.
pub fn is_valid_move(board: &Board, player: &Player, coordinates: &Coordinates) -> bool {
    // TODO: Implement the is valid move function.
    false
}

pub fn make_move(board: &mut Board, player: &Player, coordinates: &Coordinates) -> bool {
    // TODO: Implement the make move function.
    flip_disks();

    if has_valid_move(player) {
        return true;
    }
    true
}

pub fn has_valid_move(player: &Player) -> bool {
    // TODO: Implement the has valid move function.
    true
}
