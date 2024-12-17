use anyhow::anyhow;

use crate::{
    cgame::{game::Game, moves::LongAlgebraicMove},
    magic::{get_rook_magic_map, masks::get_rook_mask},
};

pub trait RookMoveGenerator {
    fn generate_pseudolegal_rook_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error>;
}

impl RookMoveGenerator for Game {
    /// This function generates all pseudolegal rook moves for the given game. The rook positions are each used to index a magic
    /// bitboard for fast calculation of moves based on blocking pieces. Does not account for pins or blocking checks. Includes queens.
    fn generate_pseudolegal_rook_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error> {
        let mut moves = Vec::new();

        let rook_pieces = self.board.rooks(&self.turn) | self.board.queens(&self.turn);
        let player_pieces = self.board.all_pieces(&self.turn);
        let opponent_pieces = self.board.all_pieces(&self.turn.other());

        let mut remaining_rooks = rook_pieces;
        while remaining_rooks != 0 {
            let current_rook = remaining_rooks & (!remaining_rooks + 1);
            let blockers = (player_pieces | opponent_pieces) & get_rook_mask(current_rook)?;

            let rook_moves = get_rook_magic_map()
                .get(current_rook.trailing_zeros() as usize)
                .ok_or(anyhow!(
                    "could not find magic bitboard for requested rook position"
                ))?
                .get(blockers)
                & !player_pieces;

            let mut remaining_rook_moves = rook_moves;
            while remaining_rook_moves != 0 {
                let dest = remaining_rook_moves & (!remaining_rook_moves + 1);
                moves.push(LongAlgebraicMove::new(current_rook, dest));
                remaining_rook_moves &= remaining_rook_moves - 1;
            }

            remaining_rooks &= remaining_rooks - 1;
        }

        Ok(moves)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculates_basic_rook_moves() {
        let game = Game::from_fen("k7/8/3p4/3r3p/8/B2N4/qb6/7K b - - 0 1").unwrap();

        let moves = game.generate_pseudolegal_rook_moves().unwrap();

        assert_eq!(moves.len(), 10);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2a1").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2a3").unwrap()));

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5c5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5b5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5a5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5e5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5f5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5g5").unwrap()));

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5d4").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5d3").unwrap()));
    }

    #[test]
    fn calculates_parallel_rook_moves() {
        let game =
            Game::from_fen("r1bq1rk1/pppn1ppp/3bpn2/3p4/2PP4/5NP1/PP2PPBP/RNBQ1RK1 w - - 0 1")
                .unwrap();

        let moves = game.generate_pseudolegal_rook_moves().unwrap();

        assert_eq!(moves.len(), 4);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d1d2").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d1d3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d1e1").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("f1e1").unwrap()));
    }
}
