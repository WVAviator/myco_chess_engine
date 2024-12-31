use arrayvec::ArrayVec;

use crate::{
    game::game::{Game, Turn},
    magic::{get_bishop_magic_map, masks::BISHOP_MASKS},
    moves::simple_move::SimpleMove,
    util::iter::{BitIndexIterable, BitIterable},
};

pub trait BishopMoveGen {
    fn generate_bishop_vision(&self, turn: &Turn, vision: &mut [u64; 8]);
    fn generate_pseudolegal_bishop_moves(&self, moves: &mut ArrayVec<SimpleMove, 256>);
}

impl BishopMoveGen for Game {
    fn generate_bishop_vision(&self, turn: &Turn, vision: &mut [u64; 8]) {
        let (bishops, queens) = match turn {
            Turn::White => (self.board.white[3], self.board.white[4]),
            Turn::Black => (self.board.black[3], self.board.black[4]),
        };

        for index in bishops.bit_indexes() {
            let blockers = self.board.all() & BISHOP_MASKS[index];
            let bishop_moves = get_bishop_magic_map()[index].get(blockers);
            vision[3] |= bishop_moves;
        }

        for index in queens.bit_indexes() {
            let blockers = self.board.all() & BISHOP_MASKS[index];
            let queen_moves = get_bishop_magic_map()[index].get(blockers);
            vision[4] |= queen_moves;
        }
    }

    fn generate_pseudolegal_bishop_moves(&self, moves: &mut ArrayVec<SimpleMove, 256>) {
        let (bishops, own_pieces) = match self.turn {
            Turn::White => (
                self.board.white[3] | self.board.white[4],
                self.board.white[6],
            ),
            Turn::Black => (
                self.board.black[3] | self.board.black[4],
                self.board.black[6],
            ),
        };

        for current_bishop in bishops.bits() {
            let index = current_bishop.trailing_zeros() as usize;
            let blockers = self.board.all() & BISHOP_MASKS[index];
            let bishop_moves = get_bishop_magic_map()[index].get(blockers) & !own_pieces;

            for dest in bishop_moves.bits() {
                moves.push(SimpleMove::new(current_bishop, dest));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculates_basic_bishop_moves() {
        let game = Game::from_fen("1k6/6p1/3r4/1q1NB2p/4BR2/8/1b6/7K w - - 0 1").unwrap();

        let mut moves = ArrayVec::new();
        game.generate_pseudolegal_bishop_moves(&mut moves);

        assert_eq!(moves.len(), 14);

        assert!(moves.contains(&SimpleMove::from_algebraic("e5d6").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e5f6").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e5g7").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e5d4").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e5c3").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e5b2").unwrap()));

        assert!(moves.contains(&SimpleMove::from_algebraic("e4f5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e4g6").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e4h7").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e4f3").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e4g2").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e4d3").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e4c2").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e4b1").unwrap()));
    }

    #[test]
    fn calculates_bishop_vision() {
        let game =
            Game::from_fen("rn2k1r1/pbpp1ppp/1p6/2b1p3/4P3/1PNP3N/PBPQBnPP/R3K2R w KQq - 0 1")
                .unwrap();
        let mut vision = [0; 8];
        let bishop_vision = game.generate_bishop_vision(&Turn::Black, &mut vision);

        assert_eq!(vision[3], 0x25100f081a112000);
        assert_eq!(vision[4], 0x25100f081a112000);
    }
}
