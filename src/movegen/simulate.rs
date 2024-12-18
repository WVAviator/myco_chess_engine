use anyhow::anyhow;

use crate::{
    cgame::{
        constants::{A_FILE, EIGHTH_RANK, FIRST_RANK, H_FILE},
        game::{Game, Turn},
        moves::LongAlgebraicMove,
    },
    magic::{
        get_bishop_magic_map, get_rook_magic_map,
        masks::{get_bishop_mask, get_rook_mask},
    },
};

use super::{king::get_king_moves, knight::get_knight_moves};

pub trait Simulate {
    fn check_move_legality(&self, lmove: &LongAlgebraicMove) -> Result<bool, anyhow::Error>;
}

impl Simulate for Game {
    fn check_move_legality(&self, lmove: &LongAlgebraicMove) -> Result<bool, anyhow::Error> {
        let mut simulated_board = self.board.clone();
        simulated_board.apply_move(lmove);

        let superking = simulated_board.king(&self.turn);
        let occupied = simulated_board.occupied();

        // Pawns
        match self.turn {
            Turn::White => {
                if ((superking & !A_FILE & !EIGHTH_RANK) << 7) & simulated_board.black_pawns != 0
                    || ((superking & !H_FILE & !EIGHTH_RANK) << 9) & simulated_board.black_pawns
                        != 0
                {
                    return Ok(false);
                }
            }
            Turn::Black => {
                if ((superking & !A_FILE & !FIRST_RANK) >> 9) & simulated_board.white_pawns != 0
                    || ((superking & !H_FILE & !FIRST_RANK) >> 7) & simulated_board.white_pawns != 0
                {
                    return Ok(false);
                }
            }
        }

        // Knights
        let knight_moves = get_knight_moves()
            .get(superking.trailing_zeros() as usize)
            .ok_or(anyhow!("could not retrieve precomputed knight moves"))?;
        if knight_moves & simulated_board.knights(&self.turn.other()) != 0 {
            return Ok(false);
        }

        // King
        let king_moves = get_king_moves()
            .get(superking.trailing_zeros() as usize)
            .ok_or(anyhow!("could not retrieve precmputed king moves"))?;
        if king_moves & simulated_board.king(&self.turn.other()) != 0 {
            return Ok(false);
        }

        // Bishops
        let bishop_moves = get_bishop_magic_map()
            .get(superking.trailing_zeros() as usize)
            .ok_or(anyhow!(
                "could not retrieve magic bitboards for bishop moves"
            ))?
            .get(get_bishop_mask(superking)? & occupied);
        if bishop_moves
            & (simulated_board.bishops(&self.turn.other())
                | simulated_board.queens(&self.turn.other()))
            != 0
        {
            return Ok(false);
        }

        // Rooks
        let rook_moves = get_rook_magic_map()
            .get(superking.trailing_zeros() as usize)
            .ok_or(anyhow!("could not retrieve magic bitboards for rook moves"))?
            .get(get_rook_mask(superking)? & occupied);
        if rook_moves
            & (simulated_board.rooks(&self.turn.other())
                | simulated_board.queens(&self.turn.other()))
            != 0
        {
            return Ok(false);
        }

        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use crate::movegen::MoveGen;

    use super::*;

    #[test]
    fn detects_absolute_pins() {
        let game =
            Game::from_fen("rn2k1r1/pbpp1ppp/1p6/4p2B/1B1PQ3/1PN4N/P1P2nPP/R3K2R b KQq - 0 1")
                .unwrap();

        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("f7f5").unwrap())
            .unwrap());
        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("e5d4").unwrap())
            .unwrap());
        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("e8e7").unwrap())
            .unwrap());

        assert!(game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("f2e4").unwrap())
            .unwrap());
        assert!(game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("g7g6").unwrap())
            .unwrap());
    }

    #[test]
    fn must_stop_check() {
        let game = Game::from_fen("r2nk1r1/pbpp1ppp/1p6/7B/1B1PQ3/1PN4N/P1P2nPP/R3K2R b KQq - 0 1")
            .unwrap();

        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("a7a5").unwrap())
            .unwrap());
        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("d8c6").unwrap())
            .unwrap());
        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("f2h1").unwrap())
            .unwrap());
        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("e8f8").unwrap())
            .unwrap());

        assert!(game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("e8d8").unwrap())
            .unwrap());
        assert!(game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("f2e4").unwrap())
            .unwrap());
        assert!(game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("b7e4").unwrap())
            .unwrap());
        assert!(game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("d8e6").unwrap())
            .unwrap());
    }

    #[test]
    fn double_checkmate_no_moves() {
        let game =
            Game::from_fen("r2nk1r1/pbpp2pp/1p4B1/8/1B1P4/1PN1Q2N/P1P2nPP/R3K2R b KQq - 0 1")
                .unwrap();

        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("h7g6").unwrap())
            .unwrap());
        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("f2e4").unwrap())
            .unwrap());
        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("e8e7").unwrap())
            .unwrap());
        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("e8f8").unwrap())
            .unwrap());
        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("d8e6").unwrap())
            .unwrap());
    }

    #[test]
    fn en_passant_stop_mate() {
        let game = Game::from_fen("k2r1r2/8/8/3pP3/4K3/2q5/8/8 w - d6 0 1").unwrap();

        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("e4d4").unwrap())
            .unwrap());
        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("e4f4").unwrap())
            .unwrap());
        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("e5e6").unwrap())
            .unwrap());

        assert!(game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("e5d6").unwrap())
            .unwrap());
    }

    #[test]
    fn no_en_passant_pinned() {
        let game = Game::from_fen("k2r1r2/6K1/8/3pP3/8/2q5/8/8 w - d6 0 1").unwrap();

        assert!(!game
            .check_move_legality(&LongAlgebraicMove::from_algebraic("e5d6").unwrap())
            .unwrap());
    }
}
