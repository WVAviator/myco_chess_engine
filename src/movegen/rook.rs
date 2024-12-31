use arrayvec::ArrayVec;

use crate::{
    game::game::{Game, Turn},
    magic::{
        get_rook_magic_map,
        masks::{get_rook_mask, ROOK_MASKS},
    },
    moves::simple_move::SimpleMove,
    util::iter::{BitIndexIterable, BitIterable},
};

pub trait RookMoveGen {
    fn generate_rook_vision(&self, turn: &Turn, vision: &mut [u64; 8]);
    fn generate_pseudolegal_rook_moves(&self, moves: &mut ArrayVec<SimpleMove, 256>);
}

impl RookMoveGen for Game {
    fn generate_rook_vision(&self, turn: &Turn, vision: &mut [u64; 8]) {
        let (rooks, queens) = match turn {
            Turn::White => (self.board.white[1], self.board.white[4]),
            Turn::Black => (self.board.black[1], self.board.black[4]),
        };

        for index in rooks.bit_indexes() {
            let blockers = self.board.all() & ROOK_MASKS[index];
            let rook_moves = get_rook_magic_map()[index].get(blockers);
            vision[1] |= rook_moves;
        }

        for index in queens.bit_indexes() {
            let blockers = self.board.all() & ROOK_MASKS[index];
            let queen_moves = get_rook_magic_map()[index].get(blockers);
            vision[4] |= queen_moves;
        }
    }

    fn generate_pseudolegal_rook_moves(&self, moves: &mut ArrayVec<SimpleMove, 256>) {
        let (rooks, own_pieces) = match self.turn {
            Turn::White => (
                self.board.white[1] | self.board.white[4],
                self.board.white[6],
            ),
            Turn::Black => (
                self.board.black[1] | self.board.black[4],
                self.board.black[6],
            ),
        };

        for current_rook in rooks.bits() {
            let index = current_rook.trailing_zeros() as usize;
            let blockers = self.board.all() & ROOK_MASKS[index];
            let rook_moves = get_rook_magic_map()[index].get(blockers) & !own_pieces;

            for dest in rook_moves.bits() {
                moves.push(SimpleMove::new(current_rook, dest));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculates_basic_rook_moves() {
        let game = Game::from_fen("k7/8/3p4/3r3p/8/B2N4/qb6/7K b - - 0 1").unwrap();

        let mut moves = ArrayVec::new();
        game.generate_pseudolegal_rook_moves(&mut moves);

        assert_eq!(moves.len(), 10);

        assert!(moves.contains(&SimpleMove::from_algebraic("a2a1").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a2a3").unwrap()));

        assert!(moves.contains(&SimpleMove::from_algebraic("d5c5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d5b5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d5a5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d5e5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d5f5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d5g5").unwrap()));

        assert!(moves.contains(&SimpleMove::from_algebraic("d5d4").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d5d3").unwrap()));
    }

    #[test]
    fn calculates_parallel_rook_moves() {
        let game =
            Game::from_fen("r1bq1rk1/pppn1ppp/3bpn2/3p4/2PP4/5NP1/PP2PPBP/RNBQ1RK1 w - - 0 1")
                .unwrap();

        let mut moves = ArrayVec::new();
        game.generate_pseudolegal_rook_moves(&mut moves);

        assert_eq!(moves.len(), 4);

        assert!(moves.contains(&SimpleMove::from_algebraic("d1d2").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d1d3").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d1e1").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("f1e1").unwrap()));
    }
}
