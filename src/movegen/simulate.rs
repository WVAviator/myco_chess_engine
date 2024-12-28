use crate::{
    game::{
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
        // Castle legality was checked during psuedolegal move gen
        match (lmove.orig & (self.board.black[5] | self.board.white[5])) | lmove.dest {
            0x5000000000000000 => return true, // Castle kingside
            0x1400000000000000 => return true, // Castle queenside
            0x50 => return true,               // Castle kingside
            0x14 => return true,               // Castle queenside
            _ => {}
        }

        // Uses a 'shadow board' and 'superking' to compute potentially illegal moves.
        // Leverages the fact that orig will always be unset and dest will always be set with the player's own piece (so it can be used as a mask for sliding pieces but omitted from any potential attacking pieces)

        let enpassant_target = lmove.en_passant_target(
            self.board.white[0] | self.board.black[0],
            self.board.empty(),
        );

        let adjusted_occupied = (self.board.all() & !lmove.orig & !enpassant_target) | lmove.dest;

        let mut attacks = 0;

        match self.turn {
            Turn::White => {
                let superking = if self.board.white[5] & lmove.orig != 0 {
                    lmove.dest
                } else {
                    self.board.white[5]
                };
                let superking_index = superking.trailing_zeros() as usize;
                // Pawns
                attacks |= ((superking & !A_FILE & !EIGHTH_RANK) << 7)
                    & (self.board.black[0] & !lmove.dest & !enpassant_target);
                attacks |= ((superking & !H_FILE & !EIGHTH_RANK) << 9)
                    & (self.board.black[0] & !lmove.dest & !enpassant_target);
                // Knights
                attacks |= KNIGHT_MOVES[superking_index] & (self.board.black[2] & !lmove.dest);
                // King
                attacks |= KING_MOVES[superking_index] & self.board.black[5];
                // Bishops
                let bishop_moves = get_bishop_magic_map()
                    .get(superking_index)
                    .expect("could not retrieve magic bitboards for bishop moves")
                    .get(get_bishop_mask(superking) & adjusted_occupied);
                attacks |=
                    bishop_moves & ((self.board.black[3] | self.board.black[4]) & !lmove.dest);
                // Rooks
                let rook_moves = get_rook_magic_map()
                    .get(superking_index)
                    .expect("could not retrieve magic bitboards for rook moves")
                    .get(get_rook_mask(superking) & adjusted_occupied);
                attacks |= rook_moves & ((self.board.black[1] | self.board.black[4]) & !lmove.dest);
            }
            Turn::Black => {
                let superking = if self.board.black[5] & lmove.orig != 0 {
                    lmove.dest
                } else {
                    self.board.black[5]
                };
                let superking_index = superking.trailing_zeros() as usize;
                // Pawns
                attacks |= ((superking & !A_FILE & !FIRST_RANK) >> 9)
                    & (self.board.white[0] & !lmove.dest & !enpassant_target);
                attacks |= ((superking & !H_FILE & !FIRST_RANK) >> 7)
                    & (self.board.white[0] & !lmove.dest & !enpassant_target);
                // Knights
                attacks |= KNIGHT_MOVES[superking_index] & (self.board.white[2] & !lmove.dest);
                // King
                attacks |= KING_MOVES[superking_index] & self.board.white[5];
                // Bishops
                let bishop_moves = get_bishop_magic_map()
                    .get(superking_index)
                    .expect("could not retrieve magic bitboards for bishop moves")
                    .get(get_bishop_mask(superking) & adjusted_occupied);
                attacks |=
                    bishop_moves & ((self.board.white[3] | self.board.white[4]) & !lmove.dest);
                // Rooks
                let rook_moves = get_rook_magic_map()
                    .get(superking_index)
                    .expect("could not retrieve magic bitboards for rook moves")
                    .get(get_rook_mask(superking) & adjusted_occupied);
                attacks |= rook_moves & ((self.board.white[1] | self.board.white[4]) & !lmove.dest);
            }
        }

        attacks == 0
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
