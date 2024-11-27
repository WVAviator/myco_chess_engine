use anyhow::{anyhow, bail, Context};

use super::{
    board::{Board, Square},
    castling_rights::{self, CastlingRights},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    board: Board,
    active_color: Color,
    castling_rights: CastlingRights,
    en_passant_target: Option<Square>,
    halfmove_clock: u32,
    fullmove_number: u32,
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
        })
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
}
