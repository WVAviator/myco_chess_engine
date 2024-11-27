use anyhow::{anyhow, bail};

use super::piece::Piece;

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
            let piece = Piece::from_fen_char(sq)?;
            board.set_index(index, Some(piece));
            index += 1;
        }

        return Ok(board);
    }

    pub fn at_square(&self, square: Square) -> &Option<Piece> {
        return self.at_index(square.0);
    }

    pub fn set_square(&mut self, square: Square, piece: Option<Piece>) {
        self.set_index(square.0, piece);
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

#[derive(Debug, Clone, PartialEq)]
pub struct Square(usize);

impl Square {
    pub fn to_algebraic(self) -> String {
        let file = (self.0 % 8) as u8 + b'a';
        let rank = 7 - (self.0 / 8) as u8 + b'1';
        format!("{}{}", file as char, rank as char)
    }

    pub fn from_algebraic(algebraic: &str) -> Result<Self, anyhow::Error> {
        if algebraic.len() != 2 {
            bail!("Invalid algebraic notation for square: {}", algebraic);
        }
        let bytes = algebraic.as_bytes();
        let file = bytes[0];
        let rank = bytes[1];

        if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
            bail!(
                "Invalid algebraic characters in square notation: {}",
                algebraic
            );
        }

        let file_index = file - b'a';
        let rank_index = 7 - (rank - b'1');

        Ok(Square((rank_index * 8 + file_index) as usize))
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

    #[test]
    fn square_from_algebraic() {
        assert_eq!(Square::from_algebraic("a8").unwrap().0, 0);
        assert_eq!(Square::from_algebraic("h8").unwrap().0, 7);
        assert_eq!(Square::from_algebraic("a1").unwrap().0, 56);
        assert_eq!(Square::from_algebraic("h1").unwrap().0, 63);
        assert_eq!(Square::from_algebraic("d5").unwrap().0, 27);
    }

    #[test]
    fn square_to_algebraic() {
        assert_eq!(Square(0).to_algebraic(), "a8");
        assert_eq!(Square(7).to_algebraic(), "h8");
        assert_eq!(Square(56).to_algebraic(), "a1");
        assert_eq!(Square(63).to_algebraic(), "h1");
        assert_eq!(Square(27).to_algebraic(), "d5");
    }

    #[test]
    fn square_invalid_algebraic_notation() {
        assert!(Square::from_algebraic("z1").is_err()); // Invalid file
        assert!(Square::from_algebraic("a9").is_err()); // Invalid rank
        assert!(Square::from_algebraic("a").is_err()); // Too short
        assert!(Square::from_algebraic("a11").is_err()); // Too long
    }

    #[test]
    fn test_round_trip() {
        let original = Square(36);
        let algebraic = original.clone().to_algebraic();
        let converted = Square::from_algebraic(&algebraic).unwrap();
        assert_eq!(original.0, converted.0);
    }
}
