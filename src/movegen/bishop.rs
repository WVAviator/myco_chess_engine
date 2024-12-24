use smallvec::SmallVec;

use crate::{
    cgame::game::{Game, Turn},
    magic::{get_bishop_magic_map, masks::get_bishop_mask},
    moves::simple_move::SimpleMove,
};

pub trait BishopMoveGen {
    fn generate_bishop_vision(&self, turn: &Turn) -> u64;
    fn generate_pseudolegal_bishop_moves(&self, moves: &mut SmallVec<[SimpleMove; 256]>);
}

impl BishopMoveGen for Game {
    fn generate_bishop_vision(&self, turn: &Turn) -> u64 {
        let mut vision = 0;

        let bishop_pieces = self.board.bishops(turn) | self.board.queens(turn);
        let player_pieces = self.board.all_pieces(turn);
        let opponent_pieces = self.board.all_pieces(&turn.other());

        let mut remaining_bishops = bishop_pieces;
        while remaining_bishops != 0 {
            let current_bishop = remaining_bishops & (!remaining_bishops + 1);
            let blockers = (player_pieces | opponent_pieces) & get_bishop_mask(current_bishop);
            let bishop_moves = get_bishop_magic_map()
                .get(current_bishop.trailing_zeros() as usize)
                .expect("could not find magic bitboard for requested bishop position")
                .get(blockers);

            vision |= bishop_moves;

            remaining_bishops &= remaining_bishops - 1;
        }

        vision
    }
    fn generate_pseudolegal_bishop_moves(&self, moves: &mut SmallVec<[SimpleMove; 256]>) {
        let bishop_pieces = self.board.bishops(&self.turn) | self.board.queens(&self.turn);
        let player_pieces = self.board.all_pieces(&self.turn);
        let opponent_pieces = self.board.all_pieces(&self.turn.other());

        let mut remaining_bishops = bishop_pieces;
        while remaining_bishops != 0 {
            let current_bishop = remaining_bishops & (!remaining_bishops + 1);
            let blockers = (player_pieces | opponent_pieces) & get_bishop_mask(current_bishop);

            let bishop_moves = get_bishop_magic_map()
                .get(current_bishop.trailing_zeros() as usize)
                .expect("could not find magic bitboard for requested bishop position")
                .get(blockers)
                & !player_pieces;

            let mut remaining_bishop_moves = bishop_moves;
            while remaining_bishop_moves != 0 {
                let dest = remaining_bishop_moves & (!remaining_bishop_moves + 1);
                moves.push(SimpleMove::new(current_bishop, dest));
                remaining_bishop_moves &= remaining_bishop_moves - 1;
            }

            remaining_bishops &= remaining_bishops - 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculates_basic_bishop_moves() {
        let game = Game::from_fen("1k6/6p1/3r4/1q1NB2p/4BR2/8/1b6/7K w - - 0 1").unwrap();

        let mut moves = SmallVec::new();
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
        let bishop_vision = game.generate_bishop_vision(&Turn::Black);

        assert_eq!(bishop_vision, 0x25100f081a112000);
    }
}
