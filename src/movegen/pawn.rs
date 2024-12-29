use smallvec::SmallVec;

use crate::{
    game::{
        constants::{A_FILE, H_FILE, SECOND_RANK, SEVENTH_RANK, SIXTH_RANK, THIRD_RANK},
        game::{Game, Turn},
    },
    moves::simple_move::SimpleMove,
};

pub trait PawnMoveGen {
    fn generate_pawn_vision(&self, turn: &Turn) -> u64;
    fn generate_psuedolegal_pawn_moves(&self, moves: &mut SmallVec<[SimpleMove; 256]>);
}

impl PawnMoveGen for Game {
    fn generate_pawn_vision(&self, turn: &Turn) -> u64 {
        let mut vision = 0;

        match turn {
            Turn::White => {
                vision |= (self.board.white[0] & !A_FILE) << 7;
                vision |= (self.board.white[0] & !H_FILE) << 9;
            }
            Turn::Black => {
                vision |= (self.board.black[0] & !A_FILE) >> 9;
                vision |= (self.board.black[0] & !H_FILE) >> 7;
            }
        }

        vision
    }
    fn generate_psuedolegal_pawn_moves(&self, moves: &mut SmallVec<[SimpleMove; 256]>) {
        match self.turn {
            Turn::White => {
                let pawns = self.board.white[0];
                let occupied = self.board.all();
                let opponent_pieces = self.board.black[6];

                let single_advance = ((pawns & !SEVENTH_RANK) << 8) & !occupied;
                backtrack_moves(single_advance, |bit| bit >> 8, moves);

                let double_advance = ((single_advance & THIRD_RANK) << 8) & !occupied;
                backtrack_moves(double_advance, |bit| bit >> 16, moves);

                let take_left =
                    ((pawns & !A_FILE & !SEVENTH_RANK) << 7) & (opponent_pieces | self.en_passant);
                backtrack_moves(take_left, |bit| bit >> 7, moves);

                let take_right =
                    ((pawns & !H_FILE & !SEVENTH_RANK) << 9) & (opponent_pieces | self.en_passant);
                backtrack_moves(take_right, |bit| bit >> 9, moves);

                let single_advance_promotion = ((pawns & SEVENTH_RANK) << 8) & !occupied;
                backtrack_moves_promotion(single_advance_promotion, |bit| bit >> 8, moves);

                let take_left_promotion =
                    ((pawns & !A_FILE & SEVENTH_RANK) << 7) & (opponent_pieces | self.en_passant);
                backtrack_moves_promotion(take_left_promotion, |bit| bit >> 7, moves);

                let take_right_promotion =
                    ((pawns & !H_FILE & SEVENTH_RANK) << 9) & (opponent_pieces | self.en_passant);
                backtrack_moves_promotion(take_right_promotion, |bit| bit >> 9, moves);
            }
            Turn::Black => {
                let pawns = self.board.black[0];
                let occupied = self.board.all();
                let opponent_pieces = self.board.white[6];

                let single_advance = ((pawns & !SECOND_RANK) >> 8) & !occupied;
                backtrack_moves(single_advance, |bit| bit << 8, moves);

                let double_advance = ((single_advance & SIXTH_RANK) >> 8) & !occupied;
                backtrack_moves(double_advance, |bit| bit << 16, moves);

                let take_left =
                    ((pawns & !A_FILE & !SECOND_RANK) >> 9) & (opponent_pieces | self.en_passant);
                backtrack_moves(take_left, |bit| bit << 9, moves);

                let take_right =
                    ((pawns & !H_FILE & !SECOND_RANK) >> 7) & (opponent_pieces | self.en_passant);
                backtrack_moves(take_right, |bit| bit << 7, moves);

                let single_advance_promotion = ((pawns & SECOND_RANK) >> 8) & !occupied;
                backtrack_moves_promotion(single_advance_promotion, |bit| bit << 8, moves);

                let take_left_promotion =
                    ((pawns & !A_FILE & SECOND_RANK) >> 9) & (opponent_pieces | self.en_passant);
                backtrack_moves_promotion(take_left_promotion, |bit| bit << 9, moves);

                let take_right_promotion =
                    ((pawns & !H_FILE & SECOND_RANK) >> 7) & (opponent_pieces | self.en_passant);
                backtrack_moves_promotion(take_right_promotion, |bit| bit << 7, moves);
            }
        }
    }
}

#[inline(always)]
fn backtrack_moves<F>(
    dest_squares: u64,
    calculate_origin: F,
    moves: &mut SmallVec<[SimpleMove; 256]>,
) where
    F: Fn(u64) -> u64,
{
    let mut bb = dest_squares;

    while bb != 0 {
        let lsb = bb & (!bb + 1);
        let origin = calculate_origin(lsb);
        let lmove = SimpleMove::new(origin, lsb);
        moves.push(lmove);
        bb &= bb - 1; // Clear the least significant bit
    }
}

#[inline(always)]
fn backtrack_moves_promotion<F>(
    dest_squares: u64,
    calculate_origin: F,
    moves: &mut SmallVec<[SimpleMove; 256]>,
) where
    F: Fn(u64) -> u64,
{
    let mut bb = dest_squares;

    while bb != 0 {
        let lsb = bb & (!bb + 1);
        let origin = calculate_origin(lsb);
        moves.extend(SimpleMove::new_promotion(origin, lsb));
        bb &= bb - 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculates_white_pawn_moves() {
        let game = Game::from_fen("1qB2bkr/PPp2p1p/6p1/2r1b1RP/4pPP1/3B4/2PPP3/NQNR2K1 w - - 0 1")
            .unwrap();
        let mut moves = SmallVec::new();
        game.generate_psuedolegal_pawn_moves(&mut moves);

        assert_eq!(moves.len(), 15);

        assert!(moves.contains(&SimpleMove::from_algebraic("a7a8q").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a7a8r").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a7a8b").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a7a8n").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a7b8q").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a7b8n").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a7b8r").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a7b8b").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("c2c3").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("c2c4").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e2e3").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("f4f5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("f4e5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("h5h6").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("h5g6").unwrap()));
    }

    #[test]
    fn calculates_black_pawn_moves() {
        let game = Game::from_fen("8/1ppp4/1P2p3/2B2k2/2K5/8/5p2/6N1 b - - 0 1").unwrap();
        let mut moves = SmallVec::new();
        game.generate_psuedolegal_pawn_moves(&mut moves);

        assert_eq!(moves.len(), 13);

        assert!(moves.contains(&SimpleMove::from_algebraic("f2f1q").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("f2f1r").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("f2f1b").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("f2f1n").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("f2g1q").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("f2g1r").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("f2g1b").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("f2g1n").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e6e5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d7d6").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d7d5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("c7c6").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("c7b6").unwrap()));
    }

    #[test]
    fn pawn_moves_cannot_wrap() {
        let game = Game::from_fen("3R1n1k/1B4pp/1p6/5p2/p7/4P1P1/PP3P1P/RN4K1 b - - 0 48").unwrap();
        let mut moves = SmallVec::new();
        game.generate_psuedolegal_pawn_moves(&mut moves);

        assert!(!moves.contains(&SimpleMove::from_algebraic("a4h2").unwrap()));
    }

    #[test]
    fn calculates_black_pawn_moves_en_passant() {
        let game = Game::from_fen("8/8/8/5k2/2K1pP2/8/8/8 b - f3 0 1").unwrap();
        let mut moves = SmallVec::new();
        game.generate_psuedolegal_pawn_moves(&mut moves);

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&SimpleMove::from_algebraic("e4f3").unwrap()));
    }
}
