use crate::game::{
    constants::{A_FILE, FILEOF, H_FILE},
    game::Game,
};

const DOUBLED_PAWN_PENALTY: i32 = 50;
const BLOCKED_PAWN_PENALTY: i32 = 50;
const ISOLATED_PAWN_PENALTY: i32 = 50;

pub trait PawnStructureEval {
    fn calculate_pawn_structure_value(&self) -> i32;
}

impl PawnStructureEval for Game {
    fn calculate_pawn_structure_value(&self) -> i32 {
        let mut eval = 0;

        // Doubled or Isolated Pawns
        let mut remaining_pawns = self.board.white[0];
        while remaining_pawns != 0 {
            let current_pawn = remaining_pawns & (!remaining_pawns + 1);

            let file = FILEOF[current_pawn.trailing_zeros() as usize];
            eval -= (file & self.board.white[0] & !current_pawn).count_ones() as i32
                * DOUBLED_PAWN_PENALTY;

            let adjacent_files = ((file & !A_FILE) >> 1) | ((file & !H_FILE) << 1);
            if adjacent_files & self.board.white[5] == 0 {
                eval -= ISOLATED_PAWN_PENALTY;
            }

            remaining_pawns &= remaining_pawns - 1;
        }

        let mut remaining_pawns = self.board.black[0];
        while remaining_pawns != 0 {
            let current_pawn = remaining_pawns & (!remaining_pawns + 1);

            let file = FILEOF[current_pawn.trailing_zeros() as usize];
            eval += (file & self.board.black[0] & !current_pawn).count_ones() as i32
                * DOUBLED_PAWN_PENALTY;

            let adjacent_files = ((file & !A_FILE) >> 1) | ((file & !H_FILE) << 1);
            if adjacent_files & self.board.black[5] == 0 {
                eval += ISOLATED_PAWN_PENALTY;
            }

            remaining_pawns &= remaining_pawns - 1;
        }

        // Blocked Pawns
        eval -= ((self.board.white[0] << 8) & self.board.black[0]).count_ones() as i32
            * BLOCKED_PAWN_PENALTY;
        eval += ((self.board.black[0] >> 8) & self.board.white[0]).count_ones() as i32
            * BLOCKED_PAWN_PENALTY;

        eval
    }
}
