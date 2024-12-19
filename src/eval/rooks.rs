use crate::cgame::{
    constants::{A_FILE, FILEOF, SECOND_RANK, SEVENTH_RANK},
    game::{Game, Turn},
};

const OPEN_FILE_ROOK_BONUS: i32 = 32;
const CONNECTED_ROOKS_BONUS: i32 = 32;
const SEVENTH_RANK_ROOK_BONUS: i32 = 48;

pub trait RookEval {
    fn calculate_rook_value(&self) -> i32;
}

impl RookEval for Game {
    fn calculate_rook_value(&self) -> i32 {
        let mut value = 0;

        // Excluding queens from this evaluation
        let white_rooks = self.board.white_rooks;
        let black_rooks = self.board.black_rooks;

        // Open file rooks

        let all_pawns = self.board.white_pawns | self.board.black_pawns;

        let mut remaining_white_rooks = white_rooks;
        while remaining_white_rooks != 0 {
            let current_rook = remaining_white_rooks & (!remaining_white_rooks + 1);
            if FILEOF[current_rook.trailing_zeros()] & all_pawns == 0 {
                value += OPEN_FILE_ROOK_BONUS;
            }
            remaining_white_rooks &= remaining_white_rooks - 1;
        }

        let mut remaining_black_rooks = black_rooks;
        while remaining_black_rooks != 0 {
            let current_rook = remaining_black_rooks & (!remaining_black_rooks + 1);
            if FILEOF[current_rook.trailing_zeros()] & all_pawns == 0 {
                value -= OPEN_FILE_ROOK_BONUS;
            }
            remaining_black_rooks &= remaining_black_rooks - 1;
        }

        // Connected rooks

        let white_rook_vision = self.generate_rook_vision(&Turn::White);
        let black_rook_vision = self.generate_rook_vision(&Turn::Black);

        if white_rook_vision & white_rooks != 0 {
            value += CONNECTED_ROOKS_BONUS;
        }

        if black_rook_vision & black_rooks != 0 {
            value -= CONNECTED_ROOKS_BONUS;
        }

        // Seventh rank rooks

        if white_rooks & SEVENTH_RANK != 0 {
            value += SEVENTH_RANK_ROOK_BONUS;
        }

        if black_rooks & SECOND_RANK != 0 {
            value -= SEVENTH_RANK_ROOK_BONUS;
        }

        value
    }
}
