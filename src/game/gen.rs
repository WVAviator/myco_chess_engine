use anyhow::anyhow;

use super::{
    cmove::CMove,
    game::{Color, Game},
    square::Square,
};

pub struct MoveGenerator<'a> {
    game: &'a Game,
}

impl<'a> MoveGenerator<'a> {
    pub fn new(game: &'a Game) -> Self {
        Self { game }
    }
    pub fn generate(&self) -> Result<Vec<Game>, anyhow::Error> {
        Ok(vec![])
    }

    fn generate_rook_moves(&self, start: Square) -> Result<Vec<CMove>, anyhow::Error> {
        let mut moves = Vec::new();
        let rook_piece = self.game.board.at_square(start).clone().ok_or(anyhow!(
            "Attempted to calculate rook moves for a nonexistent piece at {}.",
            start
        ))?;

        // Down
        for i in (start.get_row() + 1)..8 {
            let square = Square::from_position(i, start.get_col())?;
            match self.game.board.at_square(square) {
                None => moves.push(CMove::new(start, square, None)),
                Some(piece) if piece.get_color() != rook_piece.get_color() => {
                    moves.push(CMove::new(start, square, None));
                    break;
                }
                Some(_) => {
                    break;
                }
            }
        }

        // Up
        for i in (0..start.get_row()).rev() {
            let square = Square::from_position(i, start.get_col())?;
            match self.game.board.at_square(square) {
                None => moves.push(CMove::new(start, square, None)),
                Some(piece) if piece.get_color() != rook_piece.get_color() => {
                    moves.push(CMove::new(start, square, None));
                    break;
                }
                Some(_) => {
                    break;
                }
            }
        }

        // Left
        for i in (0..start.get_col()).rev() {
            let square = Square::from_position(start.get_row(), i)?;
            match self.game.board.at_square(square) {
                None => moves.push(CMove::new(start, square, None)),
                Some(piece) if piece.get_color() != rook_piece.get_color() => {
                    moves.push(CMove::new(start, square, None));
                    break;
                }
                Some(_) => {
                    break;
                }
            }
        }

        // Right
        for i in (start.get_col() + 1)..8 {
            let square = Square::from_position(start.get_row(), i)?;
            match self.game.board.at_square(square) {
                None => moves.push(CMove::new(start, square, None)),
                Some(piece) if piece.get_color() != rook_piece.get_color() => {
                    moves.push(CMove::new(start, square, None));
                    break;
                }
                Some(_) => {
                    break;
                }
            }
        }

        Ok(moves)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rook_moves_correct() {
        let game = Game::from_fen("8/5B2/8/2N2R2/5b2/8/8/K1k5 w - - 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);
        let rook_pos = Square::from_algebraic("f5").unwrap();
        let moves = move_gen.generate_rook_moves(rook_pos).unwrap();
        let valid_targets = vec![
            Square::from_algebraic("f4").unwrap(),
            Square::from_algebraic("e5").unwrap(),
            Square::from_algebraic("d5").unwrap(),
            Square::from_algebraic("f6").unwrap(),
            Square::from_algebraic("g5").unwrap(),
            Square::from_algebraic("h5").unwrap(),
        ];

        for target in valid_targets {
            let cmove = CMove::new(rook_pos, target, None);
            assert!(moves.contains(&cmove));
        }

        let invalid_targets = vec![
            Square::from_algebraic("f3").unwrap(),
            Square::from_algebraic("c5").unwrap(),
            Square::from_algebraic("f7").unwrap(),
            Square::from_algebraic("b5").unwrap(),
            Square::from_algebraic("a5").unwrap(),
        ];

        for target in invalid_targets {
            let cmove = CMove::new(rook_pos, target, None);
            assert!(!moves.contains(&cmove));
        }
    }
}
