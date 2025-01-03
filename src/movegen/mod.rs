use std::simd::Simd;

use arrayvec::ArrayVec;
use bishop::BishopMoveGen;
use king::KingMoveGen;
use knight::KnightMoveGen;
use pawn::PawnMoveGen;
use rook::RookMoveGen;
use simulate::Simulate;

use crate::{
    game::game::{Game, Turn},
    moves::simple_move::SimpleMove,
};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod rook;
pub mod simulate;

pub trait MoveGen:
    PawnMoveGen + KingMoveGen + BishopMoveGen + RookMoveGen + KnightMoveGen + Simulate
{
    fn generate_vision(&self, turn: &Turn) -> Simd<u64, 8>;
    fn generate_pseudolegal_moves(&self) -> ArrayVec<SimpleMove, 256>;
    fn generate_legal_moves(&self) -> ArrayVec<SimpleMove, 256>;
}

impl MoveGen for Game {
    fn generate_vision(&self, turn: &Turn) -> Simd<u64, 8> {
        let mut vision = [0; 8];

        self.generate_king_vision(turn, &mut vision);
        self.generate_pawn_vision(turn, &mut vision);
        self.generate_rook_vision(turn, &mut vision);
        self.generate_bishop_vision(turn, &mut vision);
        self.generate_knight_vision(turn, &mut vision);

        vision[6] = vision[0] | vision[1] | vision[2] | vision[3] | vision[4] | vision[5];

        Simd::from_array(vision)
    }
    fn generate_pseudolegal_moves(&self) -> ArrayVec<SimpleMove, 256> {
        let mut moves: ArrayVec<SimpleMove, 256> = ArrayVec::new();

        self.generate_pseudolegal_king_moves(&mut moves);
        self.generate_pseudolegal_bishop_moves(&mut moves);
        self.generate_pseudolegal_rook_moves(&mut moves);
        self.generate_psuedolegal_pawn_moves(&mut moves);
        self.generate_psuedolegal_knight_moves(&mut moves);

        moves
    }
    fn generate_legal_moves(&self) -> ArrayVec<SimpleMove, 256> {
        let mut moves = self.generate_pseudolegal_moves();
        moves.retain(|lmove| self.check_move_legality(lmove));

        moves
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn calculates_opponent_vision() {
        let game = Game::from_fen("8/8/8/8/1pbk1p2/1r3P2/3B2NK/1n1Q3R b - - 0 1").unwrap();
        let opponent_vision = game.generate_vision(&Turn::White);
        assert_eq!(opponent_vision[6], 0xf2f6dcfe);
    }

    #[test]
    fn calculates_opponent_vision_2() {
        let game = Game::from_fen("8/8/8/4k3/1pb2p2/1r3P2/6NK/1n1Q2R1 b - - 0 1").unwrap();
        let opponent_vision = game.generate_vision(&Turn::White);
        assert_eq!(opponent_vision[6], 0x8080808f8fa5cfe);
    }

    #[test]
    fn pseudolegal_move_count_correct_starting_position() {
        let game = Game::new_default();
        let moves = game.generate_pseudolegal_moves();
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn legal_move_count_correct_starting_position() {
        let game = Game::new_default();
        let moves = game.generate_legal_moves();
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn legal_move_count_correct_middle_game() {
        let game =
            Game::from_fen("r1bq1rk1/pppn1ppp/3bpn2/3p4/2PP4/5NP1/PP2PPBP/RNBQ1RK1 w - - 0 1")
                .unwrap();
        let moves = game.generate_legal_moves();
        assert_eq!(moves.len(), 34);
    }

    #[test]
    fn legal_move_count_knight_king_attack() {
        let game = Game::from_fen("8/1k6/8/3N4/5n2/4P3/6KB/r7 w - - 0 1").unwrap();
        let moves = game.generate_legal_moves();
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn legal_move_count_queen_king_attack() {
        let game = Game::from_fen("8/1k4q1/8/4N3/8/3nP3/6KB/r7 w - - 0 1").unwrap();
        let moves = game.generate_legal_moves();
        assert_eq!(moves.len(), 5);
    }

    #[test]
    fn calculates_correct_moves_advanced_position() {
        let game = Game::from_fen("r3k2r/1p1nNpp1/p2p3p/4p3/4PP1R/1N2B3/PPPQ4/2KR1B2 b kq - 0 36")
            .unwrap();
        let moves = game.generate_legal_moves();

        // A bug caused it to suggest this move in this position for some reason
        assert!(!moves.contains(&SimpleMove::from_algebraic("a1b1").unwrap()));
    }
}
