use anyhow::anyhow;

use crate::{
    cgame::{game::Game, moves::LongAlgebraicMove},
    magic::{get_bishop_magic_map, masks::get_bishop_mask},
};

pub trait BishopMoveGenerator {
    fn generate_pseudolegal_bishop_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error>;
}

impl BishopMoveGenerator for Game {
    fn generate_pseudolegal_bishop_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error> {
        let mut moves = Vec::new();

        let bishop_pieces = self.board.bishops(&self.turn) | self.board.queens(&self.turn);
        let player_pieces = self.board.all_pieces(&self.turn);
        let opponent_pieces = self.board.all_pieces(&self.turn.other());

        let mut remaining_bishops = bishop_pieces;
        while remaining_bishops != 0 {
            let current_bishop = remaining_bishops & (!remaining_bishops + 1);
            let blockers = (player_pieces | opponent_pieces) & get_bishop_mask(current_bishop)?;

            let bishop_moves = get_bishop_magic_map()
                .get(current_bishop.trailing_zeros() as usize)
                .ok_or(anyhow!(
                    "could not find magic bitboard for requested bishop position"
                ))?
                .get(blockers)
                & !player_pieces;

            let mut remaining_bishop_moves = bishop_moves;
            while remaining_bishop_moves != 0 {
                let dest = remaining_bishop_moves & (!remaining_bishop_moves + 1);
                moves.push(LongAlgebraicMove::new(current_bishop, dest));
                remaining_bishop_moves &= remaining_bishop_moves - 1;
            }

            remaining_bishops &= remaining_bishops - 1;
        }

        Ok(moves)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculates_basic_bishop_moves() {
        let game = Game::from_fen("1k6/6p1/3r4/1q1NB2p/4BR2/8/1b6/7K w - - 0 1").unwrap();

        let moves = game.generate_pseudolegal_bishop_moves().unwrap();

        assert_eq!(moves.len(), 14);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5d6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5f6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5g7").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5d4").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5c3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5b2").unwrap()));

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4f5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4g6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4h7").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4f3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4g2").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4d3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4c2").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4b1").unwrap()));
    }
}
