use crate::game::error::InvalidFENStringError;
use crate::game::piece::Piece;

pub struct Board {
    pieces: Vec<Option<Piece>>,
}

impl Board {
    pub fn from_placement_data(placement_data: &str) -> Result<Self, InvalidFENStringError> {
        let mut pieces = Vec::new();

        let ranks: Vec<&str> = placement_data.split('/').collect();
        for rank in ranks {
            for char in rank.chars() {
                if let Some(empty_squares) = char.to_digit(10) {
                    for _ in 0..empty_squares {
                        pieces.push(None);
                    }
                    continue;
                }
                let piece = Piece::from_fen_char(&char)?;
                pieces.push(Some(piece));
            }
        }

        if pieces.len() != 64 {
            return Err(InvalidFENStringError::new("Invalid number of squares/pieces in placement data."));
        }

        Ok(Board {
            pieces
        })
    }

    pub fn at_square(&self, row: u8, col: u8) -> &Option<Piece> {
        let piece_position: usize = (row * 8 + col) as usize;
        self.pieces.get(piece_position).unwrap_or(&None)
    }

    pub fn to_board_string(&self) -> String {
        self.pieces.chunks(8)
            .map(|rank| {
                let mut line = String::new();
                let mut empty_squares = 0;
                for i in 0..8 {
                    if let Some(Some(piece)) = rank.get(i) {
                        if empty_squares > 0 {
                            line.push_str(empty_squares.to_string().as_str());
                            empty_squares = 0;
                        }
                        line.push(piece.to_fen_char());
                    } else {
                        empty_squares += 1;
                    }
                }

                if empty_squares > 0 {
                    line.push_str(empty_squares.to_string().as_str());
                }

                line
            }).collect::<Vec<String>>().join("/")
    }
}

#[cfg(test)]
mod test {
    use crate::game::piece::{Color, PieceType};
    use super::*;

    #[test]
    fn properly_reads_board_string() {
        let board_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let board = Board::from_placement_data(board_string).unwrap();

        assert_eq!(&Some(Piece { color: Color::Black, piece_type: PieceType::Queen }), board.at_square(0, 3));
        assert_eq!(&Some(Piece { color: Color::Black, piece_type: PieceType::Rook }), board.at_square(0, 7));
        assert_eq!(&Some(Piece { color: Color::Black, piece_type: PieceType::Pawn }), board.at_square(1, 5));
        assert_eq!(&Some(Piece { color: Color::White, piece_type: PieceType::King }), board.at_square(7, 4));
        assert_eq!(&None, board.at_square(5, 4));
        assert_eq!(&None, board.at_square(3, 7));
    }

    #[test]
    fn reads_empty_board() {
        let board_string = "8/8/8/8/8/8/8/8";
        let board = Board::from_placement_data(board_string).unwrap();

        assert!(board.pieces.iter().all(|sq| sq == &None));
    }

    #[test]
    fn fails_invalid_length() {
        let board_string = "rnbqkbnr/pppppppp/8/7/8/8/PPPPPPPP/RNBQKBNR";
        let board = Board::from_placement_data(board_string);

        assert!(board.is_err());
    }

    #[test]
    fn converts_to_fen() {
        let board_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let board = Board::from_placement_data(board_string).unwrap();
        let output_board_string = board.to_board_string();
        assert_eq!(board_string, output_board_string);
    }
}