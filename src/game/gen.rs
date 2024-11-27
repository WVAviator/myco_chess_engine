use anyhow::bail;

use crate::game::{board::Square, cmove::CMove, game::Color, piece::Piece};

use super::{castling_rights::CastlingRights, game::Game};

pub struct MoveGenerator {}

impl MoveGenerator {
    pub fn generate(game: &Game) -> Result<Vec<Game>, anyhow::Error> {
        let mut moves = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                match (game.active_color, game.board.at_position(row, col)) {
                    (Color::White, Some(Piece::WhiteRook)) => {
                        let start = Square::from_position(row, col)?;
                        for offset_row in (row + 1)..8 {
                            match game.board.at_position(offset_row, col) {
                                None => {
                                    let cmove = CMove::new(
                                        start,
                                        Square::from_position(offset_row, col)?,
                                        None,
                                    );
                                    moves.push(cmove);
                                }
                                Some(piece) if piece.get_color() != game.active_color => {
                                    let cmove = CMove::new(
                                        start,
                                        Square::from_position(offset_row, col)?,
                                        None,
                                    );
                                    moves.push(cmove);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(vec![])
    }
}
