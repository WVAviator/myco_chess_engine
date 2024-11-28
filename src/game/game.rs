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
        let active_color_data = fen_iter.next().ok_or(anyhow!(
            "Missing active color data from FEN string: {}",
            fen
        ))?;
        let castling_rights_data = fen_iter.next().ok_or(anyhow!(
            "Missing castling rights data from FEN string: {}",
            fen
        ))?;
        let en_passant_data = fen_iter.next().ok_or(anyhow!(
            "Missing en passant target square data from FEN string: {}",
            fen
        ))?;
        let halfmove_data = fen_iter.next().ok_or(anyhow!(
            "Missing halfmove clock data from FEN string: {}",
            fen
        ))?;
        let fullmove_data = fen_iter.next().ok_or(anyhow!(
            "Missing fullmove number data from FEN string: {}",
            fen
        ))?;
        if fen_iter.count() > 0 {
            bail!("Found extraneous data in FEN string: {}", fen);
        }

        let board = Board::from_fen(piece_placement_data)?;
        let active_color = Color::from_fen(active_color_data)?;
        let castling_rights = CastlingRights::from_fen(castling_rights_data)?;
        let en_passant_target = match en_passant_data {
            "-" => None,
            algebraic => Some(Square::from_algebraic(algebraic)?),
        };
        let halfmove_clock = halfmove_data.parse::<u32>().with_context(|| {
            format!(
                "Failed to parse integer value from halfmove clock in FEN string: {}",
                fen
            )
        })?;
        let fullmove_number = fullmove_data.parse::<u32>().with_context(|| {
            format!(
                "Failed to parse integer value from fullmove number in FEN string: {}",
                fen
            )
        })?;

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
                if start.get_row() == 2 && dest.get_row() == 4 {
                    let en_passant_square = Square::from_position(3, start.get_col())?;
                    next_turn.en_passant_target = Some(en_passant_square);
                }
                true
            }
            Some(Piece::BlackPawn) => {
                if start.get_row() == 7 && dest.get_row() == 5 {
                    let en_passant_square = Square::from_position(6, start.get_col())?;
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
                let take_square = Square::from_position(5, dest.get_col())?;
                next_turn.board.set_square(take_square, None);
            }
            (Some(Piece::BlackPawn), Some(_)) => {
                let take_square = Square::from_position(4, dest.get_col())?;
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
                    .move_piece(Square::from_algebraic("h1")?, Square::from_algebraic("f1")?);
            }
            (Some(Piece::WhiteKing), start, dest)
                if start == Square::from_algebraic("e1")?
                    && dest == Square::from_algebraic("c1")? =>
            {
                next_turn
                    .board
                    .move_piece(Square::from_algebraic("a1")?, Square::from_algebraic("d1")?);
            }
            (Some(Piece::BlackKing), start, dest)
                if start == Square::from_algebraic("e8")?
                    && dest == Square::from_algebraic("g8")? =>
            {
                next_turn
                    .board
                    .move_piece(Square::from_algebraic("h8")?, Square::from_algebraic("f8")?);
            }
            (Some(Piece::BlackKing), start, dest)
                if start == Square::from_algebraic("e8")?
                    && dest == Square::from_algebraic("c8")? =>
            {
                next_turn
                    .board
                    .move_piece(Square::from_algebraic("a8")?, Square::from_algebraic("d8")?);
            }
            _ => {}
        }

        // Complete the actual move
        next_turn.board.move_piece(start, dest);

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
}
