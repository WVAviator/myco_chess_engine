use std::sync::OnceLock;

use anyhow::anyhow;

use crate::cgame::{
    castling_rights::CastlingRights,
    constants::{A_FILE, EIGHTH_RANK, FIRST_RANK, H_FILE},
    game::{Game, Turn},
    moves::{algebraic_to_u64, LongAlgebraicMove},
};

const CASTLE_WHITE_KINGSIDE_MASK: u64 = 0x60;
const CASTLE_WHITE_QUEENSIDE_MASK: u64 = 0xe;
const CASTLE_BLACK_KINGSIDE_MASK: u64 = 0x6000000000000000;
const CASTLE_BLACK_QUEENSIDE_MASK: u64 = 0xe00000000000000;

const CASTLE_CHECK_WK_MASK: u64 = 0x60;
const CASTLE_CHECK_WQ_MASK: u64 = 0xc;
const CASTLE_CHECK_BK_MASK: u64 = 0x6000000000000000;
const CASTLE_CHECK_BQ_MASK: u64 = 0xc00000000000000;

pub trait KingMoveGen {
    fn generate_pseudolegal_king_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error>;
    fn generate_king_vision(&self, turn: &Turn) -> u64;
}

impl KingMoveGen for Game {
    fn generate_king_vision(&self, turn: &Turn) -> u64 {
        let king = self.board.king(turn);
        let own_pieces = self.board.all_pieces(turn);

        // No need to include castling in king vision because it cannot attack with a castle

        get_king_moves()
            .get(king.trailing_zeros() as usize)
            .unwrap()
            & !own_pieces
    }

    fn generate_pseudolegal_king_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error> {
        let king = self.board.king(&self.turn);
        let own_pieces = self.board.all_pieces(&self.turn);
        let occupied = self.board.occupied();
        let opponent_vision = self.get_opponent_vision();

        let mut moves = Vec::new();

        let destination_squares = get_king_moves()
            .get(king.trailing_zeros() as usize)
            .ok_or(anyhow!("unable to precompute king moves"))?
            & !own_pieces
            & !opponent_vision;

        let mut remaining_destinations = destination_squares;
        while remaining_destinations != 0 {
            let next_destination = remaining_destinations & (!remaining_destinations + 1);
            moves.push(LongAlgebraicMove::new(king, next_destination));
            remaining_destinations &= remaining_destinations - 1;
        }

        // TODO: Stop castle through check

        match self.turn {
            Turn::White => {
                if self.castling_rights.is_set(CastlingRights::WHITE_KINGSIDE)
                    && occupied & CASTLE_WHITE_KINGSIDE_MASK == 0
                    && opponent_vision & CASTLE_CHECK_WK_MASK == 0
                {
                    moves.push(LongAlgebraicMove::new(
                        king,
                        algebraic_to_u64("g1").unwrap(),
                    ))
                }
                if self.castling_rights.is_set(CastlingRights::WHITE_QUEENSIDE)
                    && occupied & CASTLE_WHITE_QUEENSIDE_MASK == 0
                    && opponent_vision & CASTLE_CHECK_WQ_MASK == 0
                {
                    moves.push(LongAlgebraicMove::new(
                        king,
                        algebraic_to_u64("c1").unwrap(),
                    ))
                }
            }
            Turn::Black => {
                if self.castling_rights.is_set(CastlingRights::BLACK_KINGSIDE)
                    && occupied & CASTLE_BLACK_KINGSIDE_MASK == 0
                    && opponent_vision & CASTLE_CHECK_BK_MASK == 0
                {
                    moves.push(LongAlgebraicMove::new(
                        king,
                        algebraic_to_u64("g8").unwrap(),
                    ))
                }
                if self.castling_rights.is_set(CastlingRights::BLACK_QUEENSIDE)
                    && occupied & CASTLE_BLACK_QUEENSIDE_MASK == 0
                    && opponent_vision & CASTLE_CHECK_BQ_MASK == 0
                {
                    moves.push(LongAlgebraicMove::new(
                        king,
                        algebraic_to_u64("c8").unwrap(),
                    ))
                }
            }
        }

        Ok(moves)
    }
}

static KING_MOVES: OnceLock<Vec<u64>> = OnceLock::new();

fn generate_all_king_moves() -> Vec<u64> {
    (0..64)
        .into_iter()
        .map(|i| {
            let mut dest = 0;
            let king = 1 << i;

            dest |= (king & !A_FILE) >> 1;
            dest |= (king & !A_FILE & !EIGHTH_RANK) << 7;
            dest |= (king & !EIGHTH_RANK) << 8;
            dest |= (king & !H_FILE & !EIGHTH_RANK) << 9;
            dest |= (king & !H_FILE) << 1;
            dest |= (king & !H_FILE & !FIRST_RANK) >> 7;
            dest |= (king & !FIRST_RANK) >> 8;
            dest |= (king & !A_FILE & !FIRST_RANK) >> 9;

            dest
        })
        .collect()
}

fn get_king_moves() -> &'static Vec<u64> {
    KING_MOVES.get_or_init(|| generate_all_king_moves())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_simple_king_moves() {
        let game = Game::from_fen("8/6k1/8/8/8/1n6/KP6/8 w - - 0 1").unwrap();
        let moves = game.generate_pseudolegal_king_moves().unwrap();

        assert_eq!(moves.len(), 3);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2a3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2b3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2b1").unwrap()));
    }

    #[test]
    fn calculate_king_moves_castles_white() {
        let game =
            Game::from_fen("rn1qk1r1/pbpp1ppp/1p6/2b1p3/4P3/1PNP3N/PBPQBnPP/R3K2R w KQq - 0 1")
                .unwrap();
        let moves = game.generate_pseudolegal_king_moves().unwrap();
        LongAlgebraicMove::print_list(&moves);

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e1f1").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e1g1").unwrap()));
    }

    #[test]
    fn calculate_king_moves_castles_black_forfeit() {
        let game =
            Game::from_fen("rn1qk1r1/pbpp1ppp/1p6/2b1p3/4P3/1PNP3N/PBPQBnPP/R3K2R b KQq - 0 1")
                .unwrap();
        let moves = game.generate_pseudolegal_king_moves().unwrap();

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e8e7").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e8f8").unwrap()));
    }
}
