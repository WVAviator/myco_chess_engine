use crate::{
    cgame::{castling_rights::CastlingRights, game::Game},
    movegen::king::KING_MOVES,
};

const CENTRAL_KING_SQUARES: u64 = 0x183c7e7e3c1800;
const CASTLED_WHITE_KING_SQUARES: u64 = 0xc7;
const CASTLED_BLACK_KING_SQUARES: u64 = 0xc700000000000000;

const ENDGAME_PIECE_COUNT: u32 = 18;

const ENDGAME_CENTRAL_KING_BONUS: i32 = 0;
const ENDGAME_PAWN_DEFENSE_BONUS: i32 = 16;
const CASTLED_BONUS: i32 = 64;

// Applied for each pawn surrounding a castled king
const CASTLE_DEFENSE_BONUS: i32 = 16;

// Applied up to twice for each side castle
const CASTLE_FORFEIT_PENALTY: i32 = 32;

pub trait KingSafetyEval {
    fn calculate_king_safety_value(&self) -> i32;
}

impl KingSafetyEval for Game {
    fn calculate_king_safety_value(&self) -> i32 {
        let mut value = 0;

        // Endgame

        if self.board.get_all().count_ones() < ENDGAME_PIECE_COUNT {
            value += (self.board.white[5] & CENTRAL_KING_SQUARES).count_ones() as i32
                * ENDGAME_CENTRAL_KING_BONUS;
            value -= (self.board.black[5] & CENTRAL_KING_SQUARES).count_ones() as i32
                * ENDGAME_CENTRAL_KING_BONUS;

            value += (KING_MOVES[self.board.white[5].trailing_zeros() as usize]
                & self.board.white[0])
                .count_ones() as i32
                * ENDGAME_PAWN_DEFENSE_BONUS;
            value -= (KING_MOVES[self.board.black[5].trailing_zeros() as usize]
                & self.board.black[0])
                .count_ones() as i32
                * ENDGAME_PAWN_DEFENSE_BONUS;

            return value;
        }

        // Castle defense

        let castled_white_king = self.board.white[5] & CASTLED_WHITE_KING_SQUARES;
        let castled_black_king = self.board.black[5] & CASTLED_BLACK_KING_SQUARES;

        value += castled_white_king.count_ones() as i32 * CASTLED_BONUS;
        value -= castled_black_king.count_ones() as i32 * CASTLED_BONUS;

        if castled_white_king != 0 {
            let white_castle_defenders =
                KING_MOVES[castled_white_king.trailing_zeros() as usize] & self.board.white[0];
            value += white_castle_defenders.count_ones() as i32 * CASTLE_DEFENSE_BONUS;
        }
        if castled_black_king != 0 {
            let black_castle_defenders =
                KING_MOVES[castled_black_king.trailing_zeros() as usize] & self.board.black[0];
            value -= black_castle_defenders.count_ones() as i32 * CASTLE_DEFENSE_BONUS;
        }

        // Castle forfeit

        if castled_white_king == 0 && !self.castling_rights.is_set(CastlingRights::WHITE_KINGSIDE) {
            value -= CASTLE_FORFEIT_PENALTY;
        }

        if castled_white_king == 0 && !self.castling_rights.is_set(CastlingRights::WHITE_QUEENSIDE)
        {
            value -= CASTLE_FORFEIT_PENALTY;
        }

        if castled_black_king == 0 && !self.castling_rights.is_set(CastlingRights::BLACK_KINGSIDE) {
            value += CASTLE_FORFEIT_PENALTY;
        }

        if castled_black_king == 0 && !self.castling_rights.is_set(CastlingRights::BLACK_QUEENSIDE)
        {
            value += CASTLE_FORFEIT_PENALTY;
        }

        value
    }
}
