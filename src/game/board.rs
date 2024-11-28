use std::any;

use anyhow::{anyhow, bail};

use super::{piece::Piece, square::Square};

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pieces: [Option<Piece>; 64],
}

impl Board {
    pub fn new_empty() -> Self {
        return Board {
            pieces: vec![None; 64].try_into().unwrap(),
        };
    }

    pub fn from_fen(fen_board_str: &str) -> Result<Self, anyhow::Error> {
        let mut board = Board::new_empty();
        let merged_ranks: String = fen_board_str.split("/").collect();
        let mut index: usize = 0;

        for sq in merged_ranks.chars() {
            if sq.is_ascii_digit() {
                index += sq
                    .to_digit(10)
                    .ok_or(anyhow!("Unable to convert digit to numeric value: {}", sq))?
                    as usize;
                continue;
            }
            let piece = Piece::from_fen_char(&sq.to_string())?;
            board.set_index(index, Some(piece));
            index += 1;
        }

        return Ok(board);
    }

    pub fn move_piece(&mut self, start: Square, dest: Square) -> Result<(), anyhow::Error> {
        match (self.at_square(start), self.at_square(dest)) {
            (Some(start_piece), Some(other_piece)) => {
                if start_piece.get_color() == other_piece.get_color() {
                    bail!(
                        "Cannot take same color piece! Cannot take {} with {}",
                        other_piece,
                        start_piece
                    );
                }
            }
            (None, _) => bail!(
                "Cannot move piece: No piece located at {}",
                start.to_algebraic()
            ),
            _ => {}
        }
        let piece = self.pieces[*start].take();
        self.pieces[*dest] = piece;

        Ok(())
    }

    pub fn at_square(&self, square: Square) -> &Option<Piece> {
        return self.at_index(*square);
    }

    pub fn set_square(&mut self, square: Square, piece: Option<Piece>) {
        self.set_index(*square, piece);
    }

    pub fn at_index(&self, index: usize) -> &Option<Piece> {
        return &self.pieces[index];
    }

    pub fn at_position(&self, row: u8, col: u8) -> &Option<Piece> {
        return self.at_index((row * 8 + col) as usize);
    }

    pub fn set_index(&mut self, index: usize, piece: Option<Piece>) {
        self.pieces[index] = piece;
    }

    pub fn set_position(&mut self, row: u8, col: u8, piece: Option<Piece>) {
        self.set_index((row * 8 + col) as usize, piece);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initializes_default_setup_from_fen() {
        let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let board = Board::from_fen(default_fen).unwrap();

        assert_eq!(board.at_position(0, 0), &Some(Piece::BlackRook));
        assert_eq!(board.at_position(1, 2), &Some(Piece::BlackPawn));
        assert_eq!(board.at_position(3, 3), &None);
        assert_eq!(board.at_position(7, 4), &Some(Piece::WhiteKing));
        assert_eq!(board.at_position(7, 5), &Some(Piece::WhiteBishop));
    }

    #[test]
    fn initializes_complex_setup_from_fen() {
        let fen_str = "8/8/3r3R/2b4B/8/8/K1k5/8";
        let board = Board::from_fen(fen_str).unwrap();

        assert_eq!(board.at_position(2, 3), &Some(Piece::BlackRook));
        assert_eq!(board.at_position(2, 4), &None);
        assert_eq!(board.at_position(2, 7), &Some(Piece::WhiteRook));
        assert_eq!(board.at_position(3, 2), &Some(Piece::BlackBishop));
        assert_eq!(board.at_position(3, 7), &Some(Piece::WhiteBishop));
        assert_eq!(board.at_position(6, 0), &Some(Piece::WhiteKing));
        assert_eq!(board.at_position(6, 1), &None);
        assert_eq!(board.at_position(6, 2), &Some(Piece::BlackKing));
        assert_eq!(board.at_position(7, 5), &None);
    }
}
