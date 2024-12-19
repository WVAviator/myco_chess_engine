use std::sync::OnceLock;

use anyhow::anyhow;
use smallvec::SmallVec;

use crate::cgame::{
    castling_rights::CastlingRights,
    constants::{A_FILE, EIGHTH_RANK, FIRST_RANK, H_FILE},
    game::{Game, Turn},
    moves::{algebraic_to_u64, SimpleMove},
};

use super::MoveGen;

const CASTLE_MOVE_WK_MASK: u64 = 0x60;
const CASTLE_MOVE_WQ_MASK: u64 = 0xe;
const CASTLE_MOVE_BK_MASK: u64 = 0x6000000000000000;
const CASTLE_MOVE_BQ_MASK: u64 = 0xe00000000000000;

const CASTLE_CHECK_WK_MASK: u64 = 0x70;
const CASTLE_CHECK_WQ_MASK: u64 = 0x1c;
const CASTLE_CHECK_BK_MASK: u64 = 0x7000000000000000;
const CASTLE_CHECK_BQ_MASK: u64 = 0x1c00000000000000;

pub trait KingMoveGen {
    fn generate_pseudolegal_king_moves(&self, moves: &mut SmallVec<[SimpleMove; 256]>);
    fn generate_king_vision(&self, turn: &Turn) -> u64;
}

impl KingMoveGen for Game {
    fn generate_king_vision(&self, turn: &Turn) -> u64 {
        let king = self.board.king(turn);

        // No need to include castling in king vision because it cannot attack with a castle

        *KING_MOVES
            .get(king.trailing_zeros() as usize)
            .expect("could not find precomputed king move")
    }

    fn generate_pseudolegal_king_moves(&self, moves: &mut SmallVec<[SimpleMove; 256]>) {
        let king = self.board.king(&self.turn);
        let own_pieces = self.board.all_pieces(&self.turn);
        let occupied = self.board.occupied();
        let opponent_vision = self.generate_vision(&self.turn.other());

        let destination_squares = KING_MOVES
            .get(king.trailing_zeros() as usize)
            .expect("could not find precomputed king move")
            & !own_pieces
            & !opponent_vision;

        let mut remaining_destinations = destination_squares;
        while remaining_destinations != 0 {
            let next_destination = remaining_destinations & (!remaining_destinations + 1);
            moves.push(SimpleMove::new(king, next_destination));
            remaining_destinations &= remaining_destinations - 1;
        }

        match self.turn {
            Turn::White => {
                if self.castling_rights.is_set(CastlingRights::WHITE_KINGSIDE)
                    && occupied & CASTLE_MOVE_WK_MASK == 0
                    && opponent_vision & CASTLE_CHECK_WK_MASK == 0
                {
                    moves.push(SimpleMove::new(king, algebraic_to_u64("g1")))
                }
                if self.castling_rights.is_set(CastlingRights::WHITE_QUEENSIDE)
                    && occupied & CASTLE_MOVE_WQ_MASK == 0
                    && opponent_vision & CASTLE_CHECK_WQ_MASK == 0
                {
                    moves.push(SimpleMove::new(king, algebraic_to_u64("c1")))
                }
            }
            Turn::Black => {
                if self.castling_rights.is_set(CastlingRights::BLACK_KINGSIDE)
                    && occupied & CASTLE_MOVE_BK_MASK == 0
                    && opponent_vision & CASTLE_CHECK_BK_MASK == 0
                {
                    moves.push(SimpleMove::new(king, algebraic_to_u64("g8")))
                }
                if self.castling_rights.is_set(CastlingRights::BLACK_QUEENSIDE)
                    && occupied & CASTLE_MOVE_BQ_MASK == 0
                    && opponent_vision & CASTLE_CHECK_BQ_MASK == 0
                {
                    moves.push(SimpleMove::new(king, algebraic_to_u64("c8")))
                }
            }
        }
    }
}

pub const KING_MOVES: [u64; 64] = generate_all_king_moves();

const fn generate_all_king_moves() -> [u64; 64] {
    let mut moves = [0; 64];
    let mut i = 0;
    while i < 64 {
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

        moves[i] = dest;

        i += 1;
    }
    moves
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_simple_king_moves() {
        let game = Game::from_fen("8/6k1/8/8/8/1n6/KP6/8 w - - 0 1").unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert_eq!(moves.len(), 3);

        assert!(moves.contains(&SimpleMove::from_algebraic("a2a3").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a2b3").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a2b1").unwrap()));
    }

    #[test]
    fn calculate_king_moves_castles_white() {
        let game =
            Game::from_fen("rn1qk1r1/pbpp1ppp/1p6/2b1p3/4P3/1PNP3N/PBPQBnPP/R3K2R w KQq - 0 1")
                .unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&SimpleMove::from_algebraic("e1f1").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e1g1").unwrap()));
    }

    #[test]
    fn calculate_king_moves_castles_black_forfeit() {
        let game =
            Game::from_fen("rn1qk1r1/pbpp1ppp/1p6/2b1p3/4P3/1PNP3N/PBPQBnPP/R3K2R b KQq - 0 1")
                .unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&SimpleMove::from_algebraic("e8e7").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e8f8").unwrap()));
    }

    #[test]
    fn king_cannot_put_self_in_check() {
        let game = Game::from_fen("8/8/8/4k3/1pb2p2/1r3P2/6NK/1n1Q2R1 b - - 0 1").unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert_eq!(moves.len(), 3);

        assert!(moves.contains(&SimpleMove::from_algebraic("e5e6").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e5f5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e5f6").unwrap()));
    }

    #[test]
    fn king_cannot_castle_through_check() {
        let game = Game::from_fen("8/8/k7/6P1/2b5/8/8/4K2R w K - 0 1").unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert!(!moves.contains(&SimpleMove::from_algebraic("e1g1").unwrap()));
    }

    #[test]
    fn king_cannot_castle_into_check() {
        let game = Game::from_fen("8/8/k7/6P1/3b4/8/8/4K2R w K - 0 1").unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert!(!moves.contains(&SimpleMove::from_algebraic("e1g1").unwrap()));
    }

    #[test]
    fn king_cannot_castle_while_in_check() {
        let game = Game::from_fen("8/8/k7/6P1/1b6/8/8/4K2R w K - 0 1").unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert!(!moves.contains(&SimpleMove::from_algebraic("e1g1").unwrap()));
    }
}
