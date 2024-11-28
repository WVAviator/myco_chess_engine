use anyhow::{anyhow, bail, Context};

use super::{
    board::Board, castling_rights::CastlingRights, cmove::CMove, piece::Piece, square::Square,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub board: Board,
    pub active_color: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub moves: Vec<CMove>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn from_fen(ch: &str) -> Result<Self, anyhow::Error> {
        match ch {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => bail!("Invalid active move color '{}'", ch),
        }
    }

    pub fn to_fen(&self) -> String {
        match self {
            Color::White => String::from("w"),
            Color::Black => String::from("b"),
        }
    }
}

impl Game {
    pub fn new_default() -> Self {
        return Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    }

    pub fn from_fen(fen: &str) -> Result<Self, anyhow::Error> {
        let mut fen_iter = fen.split(" ");

        let piece_placement_data = fen_iter.next().ok_or(anyhow!(
            "Missing piece placement data from FEN string: {}",
            fen
        ))?;
        let board = Board::from_fen(piece_placement_data)?;

        let active_color_data = fen_iter.next().ok_or(anyhow!(
            "Missing active color data from FEN string: {}",
            fen
        ))?;
        let active_color = Color::from_fen(active_color_data)?;

        let castling_rights_data = fen_iter.next().ok_or(anyhow!(
            "Missing castling rights data from FEN string: {}",
            fen
        ))?;
        let castling_rights = CastlingRights::from_fen(castling_rights_data)?;

        let en_passant_data = fen_iter.next().ok_or(anyhow!(
            "Missing en passant target square data from FEN string: {}",
            fen
        ))?;
        let en_passant_target = match en_passant_data {
            "-" => None,
            algebraic => Some(Square::from_algebraic(algebraic)?),
        };

        let halfmove_data = fen_iter.next().ok_or(anyhow!(
            "Missing halfmove clock data from FEN string: {}",
            fen
        ))?;
        let halfmove_clock = halfmove_data.parse::<u32>().with_context(|| {
            format!(
                "Failed to parse integer value from halfmove clock in FEN string: {}",
                fen
            )
        })?;

        let fullmove_data = fen_iter.next().ok_or(anyhow!(
            "Missing fullmove number data from FEN string: {}",
            fen
        ))?;
        let fullmove_number = fullmove_data.parse::<u32>().with_context(|| {
            format!(
                "Failed to parse integer value from fullmove number in FEN string: {}",
                fen
            )
        })?;

        if fen_iter.count() > 0 {
            bail!("Found extraneous data in FEN string: {}", fen);
        }

        Ok(Game {
            board,
            active_color,
            castling_rights,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
            moves: Vec::new(),
        })
    }

    pub fn is_legal(&self) -> bool {
        // TODO: Check if opponent king can be taken and return false if so
        return true;
    }

    pub fn apply_move(&self, cmove: CMove) -> Result<Game, anyhow::Error> {
        let mut next_turn = self.clone();
        let start = cmove.start;
        let dest = cmove.dest;

        // Update active color and fullmove number
        next_turn.active_color = match self.active_color {
            Color::White => Color::Black,
            Color::Black => {
                next_turn.fullmove_number += 1;
                Color::White
            }
        };

        // Advance the halfmove clock for the fifty move rule, and set enpassant target
        next_turn.en_passant_target = None;
        let is_capture = next_turn.board.at_square(dest).is_some();
        let is_pawn_advance = match next_turn.board.at_square(start) {
            Some(Piece::WhitePawn) => {
                if start.get_rank() == 2 && dest.get_rank() == 4 {
                    let en_passant_square = Square::from_rank_file(3, start.get_file())?;
                    next_turn.en_passant_target = Some(en_passant_square);
                }
                true
            }
            Some(Piece::BlackPawn) => {
                if start.get_rank() == 7 && dest.get_rank() == 5 {
                    let en_passant_square = Square::from_rank_file(6, start.get_file())?;
                    next_turn.en_passant_target = Some(en_passant_square);
                }
                true
            }
            _ => false,
        };

        if !is_capture && !is_pawn_advance {
            next_turn.halfmove_clock += 1;
        } else {
            next_turn.halfmove_clock = 0;
        }

        // Handle castling unsets
        match (next_turn.board.at_square(start), start.to_algebraic()) {
            (Some(Piece::WhiteKing), _) => {
                next_turn
                    .castling_rights
                    .set(CastlingRights::WHITE_KINGSIDE, false);
                next_turn
                    .castling_rights
                    .set(CastlingRights::WHITE_QUEENSIDE, false);
            }
            (Some(Piece::BlackKing), _) => {
                next_turn
                    .castling_rights
                    .set(CastlingRights::BLACK_KINGSIDE, false);
                next_turn
                    .castling_rights
                    .set(CastlingRights::BLACK_QUEENSIDE, false);
            }
            (Some(Piece::WhiteRook), sq) if sq == "a1" => {
                next_turn
                    .castling_rights
                    .set(CastlingRights::WHITE_QUEENSIDE, false);
            }
            (Some(Piece::WhiteRook), sq) if sq == "h1" => {
                next_turn
                    .castling_rights
                    .set(CastlingRights::WHITE_KINGSIDE, false);
            }
            (Some(Piece::BlackRook), sq) if sq == "a8" => {
                next_turn
                    .castling_rights
                    .set(CastlingRights::BLACK_QUEENSIDE, false);
            }
            (Some(Piece::BlackRook), sq) if sq == "h8" => {
                next_turn
                    .castling_rights
                    .set(CastlingRights::BLACK_KINGSIDE, false);
            }
            _ => {}
        }

        // Handle enpassant takes
        match (next_turn.board.at_square(start), self.en_passant_target) {
            (Some(Piece::WhitePawn), Some(_)) => {
                let take_square = Square::from_rank_file(5, dest.get_file())?;
                next_turn.board.set_square(take_square, None);
            }
            (Some(Piece::BlackPawn), Some(_)) => {
                let take_square = Square::from_rank_file(4, dest.get_file())?;
                next_turn.board.set_square(take_square, None);
            }
            _ => {}
        }

        // Handle castling moves
        match (next_turn.board.at_square(start), start, dest) {
            (Some(Piece::WhiteKing), start, dest)
                if start == Square::from_algebraic("e1")?
                    && dest == Square::from_algebraic("g1")? =>
            {
                next_turn
                    .board
                    .move_piece(Square::from_algebraic("h1")?, Square::from_algebraic("f1")?)?;
            }
            (Some(Piece::WhiteKing), start, dest)
                if start == Square::from_algebraic("e1")?
                    && dest == Square::from_algebraic("c1")? =>
            {
                next_turn
                    .board
                    .move_piece(Square::from_algebraic("a1")?, Square::from_algebraic("d1")?)?;
            }
            (Some(Piece::BlackKing), start, dest)
                if start == Square::from_algebraic("e8")?
                    && dest == Square::from_algebraic("g8")? =>
            {
                next_turn
                    .board
                    .move_piece(Square::from_algebraic("h8")?, Square::from_algebraic("f8")?)?;
            }
            (Some(Piece::BlackKing), start, dest)
                if start == Square::from_algebraic("e8")?
                    && dest == Square::from_algebraic("c8")? =>
            {
                next_turn
                    .board
                    .move_piece(Square::from_algebraic("a8")?, Square::from_algebraic("d8")?)?;
            }
            _ => {}
        }

        // Complete the actual move
        next_turn.board.move_piece(start, dest)?;

        // Handle promotions
        match (next_turn.board.at_square(dest), dest.get_rank()) {
            (Some(Piece::WhitePawn), 8) => {
                if cmove.promotion.is_none() {
                    bail!("Advanced pawn to the last rank must provide a promotion piece.");
                }
                next_turn.board.set_square(dest, cmove.promotion)
            }
            (Some(Piece::BlackPawn), 1) => {
                if cmove.promotion.is_none() {
                    bail!("Advanced pawn to the last rank must provide a promotion piece.");
                }
                next_turn.board.set_square(dest, cmove.promotion)
            }
            _ => {}
        }

        Ok(next_turn)
    }
}

#[cfg(test)]
mod test {
    use crate::game::piece::Piece;

    use super::*;

    #[test]
    fn default_new_correct() {
        let game = Game::new_default();

        assert_eq!(
            game.board.at_square(Square::from_algebraic("a8").unwrap()),
            &Some(Piece::BlackRook)
        );
        assert_eq!(
            game.board.at_square(Square::from_algebraic("a1").unwrap()),
            &Some(Piece::WhiteRook)
        );
        assert_eq!(
            game.board.at_square(Square::from_algebraic("d8").unwrap()),
            &Some(Piece::BlackQueen)
        );
        assert_eq!(
            game.board.at_square(Square::from_algebraic("e1").unwrap()),
            &Some(Piece::WhiteKing)
        );

        assert_eq!(game.active_color, Color::White);

        assert!(game.castling_rights.is_set(CastlingRights::WHITE_KINGSIDE));
        assert!(game.castling_rights.is_set(CastlingRights::WHITE_QUEENSIDE));
        assert!(game.castling_rights.is_set(CastlingRights::BLACK_KINGSIDE));
        assert!(game.castling_rights.is_set(CastlingRights::BLACK_QUEENSIDE));

        assert_eq!(game.en_passant_target, None);

        assert_eq!(game.halfmove_clock, 0);
        assert_eq!(game.fullmove_number, 1);
    }

    #[test]
    fn apply_basic_move() {
        let game = Game::from_fen("8/k7/8/8/8/8/7K/8 w - - 14 52").unwrap();
        let cmove = CMove::from_long_algebraic("h2h3").unwrap();
        let next_turn = game.apply_move(cmove).unwrap();

        assert_eq!(next_turn.active_color, Color::Black);
        assert_eq!(next_turn.castling_rights, game.castling_rights);
        assert_eq!(next_turn.en_passant_target, None);
        assert_eq!(next_turn.fullmove_number, 52);
        assert_eq!(next_turn.halfmove_clock, 15);
        assert_eq!(
            next_turn.board,
            Board::from_fen("8/k7/8/8/8/7K/8/8").unwrap()
        );
    }

    #[test]
    fn apply_move_original_unaffected() {
        let game = Game::new_default();
        let cmove = CMove::from_long_algebraic("h2h3").unwrap();
        let next_turn = game.apply_move(cmove).unwrap();

        assert_eq!(game.active_color, Color::White);
        assert_eq!(game.fullmove_number, 1);
        assert_eq!(
            game.board,
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap()
        );
        assert_ne!(game.board, next_turn.board);
    }

    #[test]
    fn apply_move_castling_forfeit() {
        let turn1 = Game::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 20").unwrap();
        // White forfeits all castling rights by moving king
        let turn2 = turn1
            .apply_move(CMove::from_long_algebraic("e1e2").unwrap())
            .unwrap();
        assert_eq!(
            turn2.castling_rights,
            CastlingRights::from_fen("kq").unwrap()
        );
        assert_eq!(
            turn2.board,
            Board::from_fen("r3k2r/8/8/8/8/8/4K3/R6R").unwrap()
        );
        // Black forfeits queenside rights only by moving rook
        let turn3 = turn2
            .apply_move(CMove::from_long_algebraic("a8a6").unwrap())
            .unwrap();
        assert_eq!(
            turn3.castling_rights,
            CastlingRights::from_fen("k").unwrap()
        );
        assert_eq!(
            turn3.board,
            Board::from_fen("4k2r/8/r7/8/8/8/4K3/R6R").unwrap()
        );
    }

    #[test]
    fn apply_move_castling() {
        let turn1 = Game::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 20").unwrap();
        // White castles kingside
        let turn2 = turn1
            .apply_move(CMove::from_long_algebraic("e1g1").unwrap())
            .unwrap();
        assert_eq!(
            turn2.castling_rights,
            CastlingRights::from_fen("kq").unwrap()
        );
        assert_eq!(
            turn2.board,
            Board::from_fen("r3k2r/8/8/8/8/8/8/R4RK1").unwrap()
        );
        // Black castles queenside
        let turn3 = turn2
            .apply_move(CMove::from_long_algebraic("e8c8").unwrap())
            .unwrap();
        assert_eq!(
            turn3.castling_rights,
            CastlingRights::from_fen("-").unwrap()
        );
        assert_eq!(
            turn3.board,
            Board::from_fen("2kr3r/8/8/8/8/8/8/R4RK1").unwrap()
        );
    }

    #[test]
    fn apply_move_sets_en_passant_square() {
        let turn1 = Game::new_default();
        let turn2 = turn1
            .apply_move(CMove::from_long_algebraic("e2e4").unwrap())
            .unwrap();
        assert_eq!(
            turn2.en_passant_target,
            Some(Square::from_algebraic("e3").unwrap())
        );
        let turn3 = turn2
            .apply_move(CMove::from_long_algebraic("c7c5").unwrap())
            .unwrap();
        assert_eq!(
            turn3.en_passant_target,
            Some(Square::from_algebraic("c6").unwrap())
        );
        let turn4 = turn3
            .apply_move(CMove::from_long_algebraic("b1c3").unwrap())
            .unwrap();
        assert_eq!(turn4.en_passant_target, None);
    }

    #[test]
    fn apply_move_en_passant_white_takes() {
        let turn1 = Game::from_fen("K7/8/8/2pP4/8/8/8/7k w - c6 0 50").unwrap();
        let turn2 = turn1
            .apply_move(CMove::from_long_algebraic("d5c6").unwrap())
            .unwrap();
        assert_eq!(turn2.board, Board::from_fen("K7/8/2P5/8/8/8/8/7k").unwrap());
        assert_eq!(turn2.en_passant_target, None);
    }

    #[test]
    fn apply_move_en_passant_black_takes() {
        let turn1 = Game::from_fen("K7/8/8/8/Pp6/8/8/7k b - a3 0 50").unwrap();
        let turn2 = turn1
            .apply_move(CMove::from_long_algebraic("b4a3").unwrap())
            .unwrap();
        assert_eq!(turn2.board, Board::from_fen("K7/8/8/8/8/p7/8/7k").unwrap());
        assert_eq!(turn2.en_passant_target, None);
    }

    #[test]
    fn apply_move_handles_promotion() {
        let turn1 = Game::from_fen("K7/6P1/8/8/8/8/8/7k w - - 0 50").unwrap();
        let turn2 = turn1
            .apply_move(CMove::from_long_algebraic("g7g8q").unwrap())
            .unwrap();
        assert_eq!(turn2.board, Board::from_fen("K5Q1/8/8/8/8/8/8/7k").unwrap());
    }

    #[test]
    fn apply_move_handles_underpromotion() {
        let turn1 = Game::from_fen("K7/8/8/8/8/8/p7/7k b - - 0 50").unwrap();
        let turn2 = turn1
            .apply_move(CMove::from_long_algebraic("a2a1n").unwrap())
            .unwrap();
        assert_eq!(turn2.board, Board::from_fen("K7/8/8/8/8/8/8/n6k").unwrap());
    }

    #[test]
    fn apply_move_fails_without_promotion() {
        let turn1 = Game::from_fen("K7/8/8/8/8/8/p7/7k b - - 0 50").unwrap();
        let turn2_result = turn1.apply_move(CMove::from_long_algebraic("a2a1").unwrap());
        assert!(turn2_result.is_err());
    }
}
