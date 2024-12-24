use crate::{
    cgame::{
        constants::{A_FILE, EIGHTH_RANK, FIRST_RANK, H_FILE},
        game::{Game, Turn},
    },
    magic::{
        get_bishop_magic_map, get_rook_magic_map,
        masks::{get_bishop_mask, get_rook_mask},
    },
    moves::simple_move::SimpleMove,
};

use super::{king::KING_MOVES, knight::KNIGHT_MOVES};

pub trait Simulate {
    fn check_move_legality(&self, lmove: &SimpleMove) -> bool;
}

impl Simulate for Game {
    fn check_move_legality(&self, lmove: &SimpleMove) -> bool {
        let mut simulated_board = self.board.clone();
        simulated_board.apply_move(lmove);

        let superking = simulated_board.king(&self.turn);
        let superking_index = superking.trailing_zeros() as usize;
        let occupied = simulated_board.occupied();

        // Pawns
        match self.turn {
            Turn::White => {
                if ((superking & !A_FILE & !EIGHTH_RANK) << 7) & simulated_board.black_pawns != 0
                    || ((superking & !H_FILE & !EIGHTH_RANK) << 9) & simulated_board.black_pawns
                        != 0
                {
                    return false;
                }
            }
            Turn::Black => {
                if ((superking & !A_FILE & !FIRST_RANK) >> 9) & simulated_board.white_pawns != 0
                    || ((superking & !H_FILE & !FIRST_RANK) >> 7) & simulated_board.white_pawns != 0
                {
                    return false;
                }
            }
        }

        // Knights
        let knight_moves = KNIGHT_MOVES
            .get(superking_index)
            .expect("coulf not retrieve precomputed knight move");
        if knight_moves & simulated_board.knights(&self.turn.other()) != 0 {
            return false;
        }

        // King
        let king_moves = KING_MOVES
            .get(superking_index)
            .expect("coulf not retrieve precomputed king move");
        if king_moves & simulated_board.king(&self.turn.other()) != 0 {
            return false;
        }

        // Bishops
        let bishop_moves = get_bishop_magic_map()
            .get(superking_index)
            .expect("could not retrieve magic bitboards for bishop moves")
            .get(get_bishop_mask(superking) & occupied);
        if bishop_moves
            & (simulated_board.bishops(&self.turn.other())
                | simulated_board.queens(&self.turn.other()))
            != 0
        {
            return false;
        }

        // Rooks
        let rook_moves = get_rook_magic_map()
            .get(superking_index)
            .expect("could not retrieve magic bitboards for rook moves")
            .get(get_rook_mask(superking) & occupied);
        if rook_moves
            & (simulated_board.rooks(&self.turn.other())
                | simulated_board.queens(&self.turn.other()))
            != 0
        {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detects_absolute_pins() {
        let game =
            Game::from_fen("rn2k1r1/pbpp1ppp/1p6/4p2B/1B1PQ3/1PN4N/P1P2nPP/R3K2R b KQq - 0 1")
                .unwrap();

        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("f7f5").unwrap()));
        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("e5d4").unwrap()));
        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("e8e7").unwrap()));

        assert!(game.check_move_legality(&SimpleMove::from_algebraic("f2e4").unwrap()));
        assert!(game.check_move_legality(&SimpleMove::from_algebraic("g7g6").unwrap()));
    }

    #[test]
    fn must_stop_check() {
        let game = Game::from_fen("r2nk1r1/pbpp1ppp/1p6/7B/1B1PQ3/1PN4N/P1P2nPP/R3K2R b KQq - 0 1")
            .unwrap();

        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("a7a5").unwrap()));
        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("d8c6").unwrap()));
        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("f2h1").unwrap()));
        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("e8f8").unwrap()));

        assert!(game.check_move_legality(&SimpleMove::from_algebraic("e8d8").unwrap()));
        assert!(game.check_move_legality(&SimpleMove::from_algebraic("f2e4").unwrap()));
        assert!(game.check_move_legality(&SimpleMove::from_algebraic("b7e4").unwrap()));
        assert!(game.check_move_legality(&SimpleMove::from_algebraic("d8e6").unwrap()));
    }

    #[test]
    fn double_checkmate_no_moves() {
        let game =
            Game::from_fen("r2nk1r1/pbpp2pp/1p4B1/8/1B1P4/1PN1Q2N/P1P2nPP/R3K2R b KQq - 0 1")
                .unwrap();

        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("h7g6").unwrap()));
        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("f2e4").unwrap()));
        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("e8e7").unwrap()));
        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("e8f8").unwrap()));
        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("d8e6").unwrap()));
    }

    #[test]
    fn en_passant_stop_mate() {
        let game = Game::from_fen("k2r1r2/8/8/3pP3/4K3/2q5/8/8 w - d6 0 1").unwrap();

        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("e4d4").unwrap()));
        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("e4f4").unwrap()));
        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("e5e6").unwrap()));

        assert!(game.check_move_legality(&SimpleMove::from_algebraic("e5d6").unwrap()));
    }

    #[test]
    fn no_en_passant_pinned() {
        let game = Game::from_fen("k2r1r2/6K1/8/3pP3/8/2q5/8/8 w - d6 0 1").unwrap();

        assert!(!game.check_move_legality(&SimpleMove::from_algebraic("e5d6").unwrap()));
    }

    #[test]
    fn simluate_move_false_if_illegal() {
        let game = Game::from_fen("8/8/4k3/8/5N2/8/1K6/4R3 b - - 0 1").unwrap();
        let lmove = SimpleMove::from_algebraic("e6e7").unwrap();
        assert_eq!(game.check_move_legality(&lmove), false);
    }

    #[test]
    fn simluate_move_true_if_legal() {
        let game = Game::from_fen("8/8/4k3/8/5N2/8/1K6/4R3 b - - 0 1").unwrap();
        let lmove = SimpleMove::from_algebraic("e6f6").unwrap();
        assert_eq!(game.check_move_legality(&lmove), true);
    }

    #[test]
    fn simluate_move_true_if_blocks_check() {
        let game = Game::from_fen("8/8/4k3/8/8/3b4/1K6/4R3 b - - 0 1").unwrap();
        let block_check = SimpleMove::from_algebraic("d3e4").unwrap();
        let allow_check = SimpleMove::from_algebraic("d3f5").unwrap();
        assert_eq!(game.check_move_legality(&block_check), true);
        assert_eq!(game.check_move_legality(&allow_check), false);
    }
}
