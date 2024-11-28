use anyhow::{anyhow, bail};

use super::{
    castling_rights::{self, CastlingRights},
    cmove::CMove,
    game::{Color, Game},
    piece::Piece,
    square::Square,
};

pub struct MoveGenerator<'a> {
    game: &'a Game,
}

impl<'a> MoveGenerator<'a> {
    pub fn new(game: &'a Game) -> Self {
        Self { game }
    }
    pub fn generate_moves(&self) -> Result<Vec<CMove>, anyhow::Error> {
        let mut moves = Vec::new();
        for i in 0..64 {
            match self.game.board.at_index(i) {
                Some(Piece::WhiteRook) if self.game.active_color == Color::White => {
                    moves.extend(self.generate_rook_moves(Square::from_index(i)?)?);
                }
                Some(Piece::WhiteKnight) if self.game.active_color == Color::White => {
                    moves.extend(self.generate_knight_moves(Square::from_index(i)?)?);
                }
                Some(Piece::WhiteBishop) if self.game.active_color == Color::White => {
                    moves.extend(self.generate_bishop_moves(Square::from_index(i)?)?);
                }
                Some(Piece::WhiteQueen) if self.game.active_color == Color::White => {
                    moves.extend(self.generate_queen_moves(Square::from_index(i)?)?);
                }
                Some(Piece::WhiteKing) if self.game.active_color == Color::White => {
                    moves.extend(self.generate_king_moves(Square::from_index(i)?)?);
                }
                Some(Piece::WhitePawn) if self.game.active_color == Color::White => {
                    moves.extend(self.generate_pawn_moves(Square::from_index(i)?)?);
                }
                Some(Piece::BlackRook) if self.game.active_color == Color::Black => {
                    moves.extend(self.generate_rook_moves(Square::from_index(i)?)?);
                }
                Some(Piece::BlackKnight) if self.game.active_color == Color::Black => {
                    moves.extend(self.generate_knight_moves(Square::from_index(i)?)?);
                }
                Some(Piece::BlackBishop) if self.game.active_color == Color::Black => {
                    moves.extend(self.generate_bishop_moves(Square::from_index(i)?)?);
                }
                Some(Piece::BlackQueen) if self.game.active_color == Color::Black => {
                    moves.extend(self.generate_queen_moves(Square::from_index(i)?)?);
                }
                Some(Piece::BlackKing) if self.game.active_color == Color::Black => {
                    moves.extend(self.generate_king_moves(Square::from_index(i)?)?);
                }
                Some(Piece::BlackPawn) if self.game.active_color == Color::Black => {
                    moves.extend(self.generate_pawn_moves(Square::from_index(i)?)?);
                }
                _ => {}
            }
        }
        Ok(moves)
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

    fn generate_bishop_moves(&self, start: Square) -> Result<Vec<CMove>, anyhow::Error> {
        let mut moves = Vec::new();
        let bishop_piece = self.game.board.at_square(start).clone().ok_or(anyhow!(
            "Attempted to calculate bishop moves for a nonexistent piece at {}.",
            start
        ))?;

        // Down Right
        let mut offset_row = start.get_row() + 1;
        let mut offset_col = start.get_col() + 1;
        while offset_row < 8 && offset_col < 8 {
            let square = Square::from_position(offset_row, offset_col)?;
            match self.game.board.at_square(square) {
                None => moves.push(CMove::new(start, square, None)),
                Some(piece) if piece.get_color() != bishop_piece.get_color() => {
                    moves.push(CMove::new(start, square, None));
                    break;
                }
                Some(_) => break,
            }
            offset_row += 1;
            offset_col += 1;
        }

        // Up Right
        let mut offset_row = start.get_row() as isize - 1;
        let mut offset_col = start.get_col() as isize + 1;
        while offset_row >= 0 && offset_col < 8 {
            let square = Square::from_position(offset_row as u8, offset_col as u8)?;
            match self.game.board.at_square(square) {
                None => moves.push(CMove::new(start, square, None)),
                Some(piece) if piece.get_color() != bishop_piece.get_color() => {
                    moves.push(CMove::new(start, square, None));
                    break;
                }
                Some(_) => break,
            }
            offset_row -= 1;
            offset_col += 1;
        }

        // Down Left
        let mut offset_row = (start.get_row() + 1) as isize;
        let mut offset_col = (start.get_col() - 1) as isize;
        while offset_row < 8 && offset_col >= 0 {
            let square = Square::from_position(offset_row as u8, offset_col as u8)?;
            match self.game.board.at_square(square) {
                None => moves.push(CMove::new(start, square, None)),
                Some(piece) if piece.get_color() != bishop_piece.get_color() => {
                    moves.push(CMove::new(start, square, None));
                    break;
                }
                Some(_) => break,
            }
            offset_row += 1;
            offset_col -= 1;
        }

        // Up Left
        let mut offset_row = start.get_row() as isize - 1;
        let mut offset_col = start.get_col() as isize - 1;
        while offset_row >= 0 && offset_col >= 0 {
            let square = Square::from_position(offset_row as u8, offset_col as u8)?;
            match self.game.board.at_square(square) {
                None => moves.push(CMove::new(start, square, None)),
                Some(piece) if piece.get_color() != bishop_piece.get_color() => {
                    moves.push(CMove::new(start, square, None));
                    break;
                }
                Some(_) => break,
            }
            offset_row -= 1;
            offset_col -= 1;
        }

        Ok(moves)
    }

    fn generate_queen_moves(&self, start: Square) -> Result<Vec<CMove>, anyhow::Error> {
        let mut moves = self.generate_rook_moves(start)?;
        moves.extend(self.generate_bishop_moves(start)?);
        Ok(moves)
    }

    fn generate_knight_moves(&self, start: Square) -> Result<Vec<CMove>, anyhow::Error> {
        let mut moves = Vec::new();
        let knight_piece = self.game.board.at_square(start).clone().ok_or(anyhow!(
            "Attempted to calculate knight moves for a nonexistent piece at {}.",
            start
        ))?;

        let knight_offsets = vec![
            (2, 1),
            (2, -1),
            (1, 2),
            (1, -2),
            (-2, 1),
            (-2, -1),
            (-1, 2),
            (-1, -2),
        ];

        for (row_offset, col_offset) in knight_offsets {
            let row = start.get_row() as isize + row_offset;
            let col = start.get_col() as isize + col_offset;
            if row < 0 || row >= 8 || col < 0 || col >= 8 {
                continue;
            }

            let square = Square::from_position(row as u8, col as u8)?;

            match self.game.board.at_square(square) {
                None => moves.push(CMove::new(start, square, None)),
                Some(piece) if piece.get_color() != knight_piece.get_color() => {
                    moves.push(CMove::new(start, square, None));
                }
                Some(_) => continue,
            }
        }

        Ok(moves)
    }

    fn generate_king_moves(&self, start: Square) -> Result<Vec<CMove>, anyhow::Error> {
        let mut moves = Vec::new();
        let king_piece = self.game.board.at_square(start).clone().ok_or(anyhow!(
            "Attempted to calculate king moves for a nonexistent piece at {}.",
            start
        ))?;

        let king_offsets = vec![
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, -1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        for (row_offset, col_offset) in king_offsets {
            let row = start.get_row() as isize + row_offset;
            let col = start.get_col() as isize + col_offset;
            if row < 0 || row >= 8 || col < 0 || col >= 8 {
                continue;
            }

            let square = Square::from_position(row as u8, col as u8)?;

            match self.game.board.at_square(square) {
                None => moves.push(CMove::new(start, square, None)),
                Some(piece) if piece.get_color() != king_piece.get_color() => {
                    moves.push(CMove::new(start, square, None));
                }
                Some(_) => continue,
            }
        }

        // Castling
        // TODO: Verify not in check or through check
        match &king_piece {
            Piece::WhiteKing => {
                if self
                    .game
                    .castling_rights
                    .is_set(CastlingRights::WHITE_KINGSIDE)
                {
                    let bishop_square = Square::from_algebraic("f1")?;
                    let knight_square = Square::from_algebraic("g1")?;
                    if self.game.board.at_square(bishop_square).is_none()
                        && self.game.board.at_square(knight_square).is_none()
                    {
                        moves.push(CMove::new(start, knight_square, None));
                    }
                }
                if self
                    .game
                    .castling_rights
                    .is_set(CastlingRights::WHITE_QUEENSIDE)
                {
                    let queen_square = Square::from_algebraic("d1")?;
                    let bishop_square = Square::from_algebraic("c1")?;
                    let knight_square = Square::from_algebraic("b1")?;
                    if self.game.board.at_square(queen_square).is_none()
                        && self.game.board.at_square(bishop_square).is_none()
                        && self.game.board.at_square(knight_square).is_none()
                    {
                        moves.push(CMove::new(start, bishop_square, None));
                    }
                }
            }
            Piece::BlackKing => {
                if self
                    .game
                    .castling_rights
                    .is_set(CastlingRights::BLACK_KINGSIDE)
                {
                    let bishop_square = Square::from_algebraic("f8")?;
                    let knight_square = Square::from_algebraic("g8")?;
                    if self.game.board.at_square(bishop_square).is_none()
                        && self.game.board.at_square(knight_square).is_none()
                    {
                        moves.push(CMove::new(start, knight_square, None));
                    }
                }
                if self
                    .game
                    .castling_rights
                    .is_set(CastlingRights::BLACK_QUEENSIDE)
                {
                    let queen_square = Square::from_algebraic("d8")?;
                    let bishop_square = Square::from_algebraic("c8")?;
                    let knight_square = Square::from_algebraic("b8")?;
                    if self.game.board.at_square(queen_square).is_none()
                        && self.game.board.at_square(bishop_square).is_none()
                        && self.game.board.at_square(knight_square).is_none()
                    {
                        moves.push(CMove::new(start, bishop_square, None));
                    }
                }
            }
            _ => bail!("Cannot apply king moves to a {}", king_piece),
        }

        Ok(moves)
    }

    fn generate_pawn_moves(&self, start: Square) -> Result<Vec<CMove>, anyhow::Error> {
        let mut moves = Vec::new();
        let pawn_piece = self.game.board.at_square(start).clone().ok_or(anyhow!(
            "Attempted to calculate pawn moves for a nonexistent piece at {}.",
            start
        ))?;

        match (&pawn_piece, start.get_rank()) {
            (Piece::WhitePawn, 2) => {
                let advance_square_1 = Square::from_position(start.get_row() + 1, start.get_col())?;
                let advance_square_2 = Square::from_position(start.get_row() + 2, start.get_col())?;
                match (
                    self.game.board.at_square(advance_square_1).is_some(),
                    self.game.board.at_square(advance_square_2).is_some(),
                ) {
                    (true, _) => {}
                    (false, true) => moves.push(CMove::new(start, advance_square_1, None)),
                    (false, false) => {
                        moves.push(CMove::new(start, advance_square_1, None));
                        moves.push(CMove::new(start, advance_square_2, None));
                    }
                }

                if start.get_file() != 'a' {
                    let take_left_square =
                        Square::from_position(start.get_row() + 1, start.get_col() - 1)?;
                    match self.game.board.at_square(take_left_square) {
                        Some(piece) if piece.get_color() != pawn_piece.get_color() => {
                            moves.push(CMove::new(start, take_left_square, None));
                        }
                        _ => {}
                    }
                }

                if start.get_file() != 'h' {
                    let take_right_square =
                        Square::from_position(start.get_row() + 1, start.get_col() + 1)?;
                    match self.game.board.at_square(take_right_square) {
                        Some(piece) if piece.get_color() != pawn_piece.get_color() => {
                            moves.push(CMove::new(start, take_right_square, None));
                        }
                        _ => {}
                    }
                }
            }
            (Piece::WhitePawn, 7) => {
                let advance_square = Square::from_position(start.get_row() + 1, start.get_col())?;
                if self.game.board.at_square(advance_square).is_none() {
                    moves.push(CMove::new(start, advance_square, Some(Piece::WhiteQueen)));
                    moves.push(CMove::new(start, advance_square, Some(Piece::WhiteRook)));
                    moves.push(CMove::new(start, advance_square, Some(Piece::WhiteBishop)));
                    moves.push(CMove::new(start, advance_square, Some(Piece::WhiteKnight)));
                }

                if start.get_file() != 'a' {
                    let take_left_square =
                        Square::from_position(start.get_row() + 1, start.get_col() - 1)?;
                    match self.game.board.at_square(take_left_square) {
                        Some(piece) if piece.get_color() != pawn_piece.get_color() => {
                            moves.push(CMove::new(
                                start,
                                take_left_square,
                                Some(Piece::WhiteQueen),
                            ));
                            moves.push(CMove::new(start, take_left_square, Some(Piece::WhiteRook)));
                            moves.push(CMove::new(
                                start,
                                take_left_square,
                                Some(Piece::WhiteBishop),
                            ));
                            moves.push(CMove::new(
                                start,
                                take_left_square,
                                Some(Piece::WhiteKnight),
                            ));
                        }
                        _ => {}
                    }
                }

                if start.get_file() != 'h' {
                    let take_right_square =
                        Square::from_position(start.get_row() + 1, start.get_col() + 1)?;
                    match self.game.board.at_square(take_right_square) {
                        Some(piece) if piece.get_color() != pawn_piece.get_color() => {
                            moves.push(CMove::new(
                                start,
                                take_right_square,
                                Some(Piece::WhiteQueen),
                            ));
                            moves.push(CMove::new(
                                start,
                                take_right_square,
                                Some(Piece::WhiteRook),
                            ));
                            moves.push(CMove::new(
                                start,
                                take_right_square,
                                Some(Piece::WhiteBishop),
                            ));
                            moves.push(CMove::new(
                                start,
                                take_right_square,
                                Some(Piece::WhiteKnight),
                            ));
                        }
                        _ => {}
                    }
                }
            }
            (Piece::WhitePawn, _) => {
                let advance_square = Square::from_position(start.get_row() + 1, start.get_col())?;
                if self.game.board.at_square(advance_square).is_none() {
                    moves.push(CMove::new(start, advance_square, None));
                }

                if start.get_file() != 'a' {
                    let take_left_square =
                        Square::from_position(start.get_row() + 1, start.get_col() - 1)?;
                    match self.game.board.at_square(take_left_square) {
                        Some(piece) if piece.get_color() != pawn_piece.get_color() => {
                            moves.push(CMove::new(start, take_left_square, None));
                        }
                        _ => {}
                    }
                    if let Some(en_passant_square) = self.game.en_passant_target {
                        if en_passant_square == take_left_square {
                            moves.push(CMove::new(start, en_passant_square, None));
                        }
                    }
                }

                if start.get_file() != 'h' {
                    let take_right_square =
                        Square::from_position(start.get_row() + 1, start.get_col() + 1)?;
                    match self.game.board.at_square(take_right_square) {
                        Some(piece) if piece.get_color() != pawn_piece.get_color() => {
                            moves.push(CMove::new(start, take_right_square, None));
                        }
                        _ => {}
                    }
                    if let Some(en_passant_square) = self.game.en_passant_target {
                        if en_passant_square == take_right_square {
                            moves.push(CMove::new(start, en_passant_square, None));
                        }
                    }
                }
            }
            (Piece::BlackPawn, 7) => {
                let advance_square_1 = Square::from_position(start.get_row() - 1, start.get_col())?;
                let advance_square_2 = Square::from_position(start.get_row() - 2, start.get_col())?;
                match (
                    self.game.board.at_square(advance_square_1).is_some(),
                    self.game.board.at_square(advance_square_2).is_some(),
                ) {
                    (true, _) => {}
                    (false, true) => moves.push(CMove::new(start, advance_square_1, None)),
                    (false, false) => {
                        moves.push(CMove::new(start, advance_square_1, None));
                        moves.push(CMove::new(start, advance_square_2, None));
                    }
                }

                if start.get_file() != 'a' {
                    let take_left_square =
                        Square::from_position(start.get_row() - 1, start.get_col() - 1)?;
                    match self.game.board.at_square(take_left_square) {
                        Some(piece) if piece.get_color() != pawn_piece.get_color() => {
                            moves.push(CMove::new(start, take_left_square, None));
                        }
                        _ => {}
                    }
                }

                if start.get_file() != 'h' {
                    let take_right_square =
                        Square::from_position(start.get_row() - 1, start.get_col() + 1)?;
                    match self.game.board.at_square(take_right_square) {
                        Some(piece) if piece.get_color() != pawn_piece.get_color() => {
                            moves.push(CMove::new(start, take_right_square, None));
                        }
                        _ => {}
                    }
                }
            }
            (Piece::BlackPawn, 2) => {
                let advance_square = Square::from_position(start.get_row() - 1, start.get_col())?;
                if self.game.board.at_square(advance_square).is_none() {
                    moves.push(CMove::new(start, advance_square, Some(Piece::BlackQueen)));
                    moves.push(CMove::new(start, advance_square, Some(Piece::BlackRook)));
                    moves.push(CMove::new(start, advance_square, Some(Piece::BlackBishop)));
                    moves.push(CMove::new(start, advance_square, Some(Piece::BlackKnight)));
                }

                if start.get_file() != 'a' {
                    let take_left_square =
                        Square::from_position(start.get_row() - 1, start.get_col() - 1)?;
                    match self.game.board.at_square(take_left_square) {
                        Some(piece) if piece.get_color() != pawn_piece.get_color() => {
                            moves.push(CMove::new(
                                start,
                                take_left_square,
                                Some(Piece::BlackQueen),
                            ));
                            moves.push(CMove::new(start, take_left_square, Some(Piece::BlackRook)));
                            moves.push(CMove::new(
                                start,
                                take_left_square,
                                Some(Piece::BlackBishop),
                            ));
                            moves.push(CMove::new(
                                start,
                                take_left_square,
                                Some(Piece::BlackKnight),
                            ));
                        }
                        _ => {}
                    }
                }

                if start.get_file() != 'h' {
                    let take_right_square =
                        Square::from_position(start.get_row() - 1, start.get_col() + 1)?;
                    match self.game.board.at_square(take_right_square) {
                        Some(piece) if piece.get_color() != pawn_piece.get_color() => {
                            moves.push(CMove::new(
                                start,
                                take_right_square,
                                Some(Piece::BlackQueen),
                            ));
                            moves.push(CMove::new(
                                start,
                                take_right_square,
                                Some(Piece::BlackRook),
                            ));
                            moves.push(CMove::new(
                                start,
                                take_right_square,
                                Some(Piece::BlackBishop),
                            ));
                            moves.push(CMove::new(
                                start,
                                take_right_square,
                                Some(Piece::BlackKnight),
                            ));
                        }
                        _ => {}
                    }
                }
            }
            (Piece::BlackPawn, _) => {
                let advance_square = Square::from_position(start.get_row() - 1, start.get_col())?;
                if self.game.board.at_square(advance_square).is_none() {
                    moves.push(CMove::new(start, advance_square, None));
                }

                if start.get_file() != 'a' {
                    let take_left_square =
                        Square::from_position(start.get_row() - 1, start.get_col() - 1)?;
                    match self.game.board.at_square(take_left_square) {
                        Some(piece) if piece.get_color() != pawn_piece.get_color() => {
                            moves.push(CMove::new(start, take_left_square, None));
                        }
                        _ => {}
                    }
                    if let Some(en_passant_square) = self.game.en_passant_target {
                        if en_passant_square == take_left_square {
                            moves.push(CMove::new(start, en_passant_square, None));
                        }
                    }
                }

                if start.get_file() != 'h' {
                    let take_right_square =
                        Square::from_position(start.get_row() - 1, start.get_col() + 1)?;
                    match self.game.board.at_square(take_right_square) {
                        Some(piece) if piece.get_color() != pawn_piece.get_color() => {
                            moves.push(CMove::new(start, take_right_square, None));
                        }
                        _ => {}
                    }
                    if let Some(en_passant_square) = self.game.en_passant_target {
                        if en_passant_square == take_right_square {
                            moves.push(CMove::new(start, en_passant_square, None));
                        }
                    }
                }
            }
            _ => bail!(
                "Attempted to calculate pawn moves for a piece that is not a pawn: {}.",
                &pawn_piece
            ),
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

        assert_eq!(moves.len(), 6);

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
    }

    #[test]
    fn bishop_moves_correct() {
        let game = Game::from_fen("K7/8/8/8/6R1/3N4/4b3/k4r2 b - - 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);
        let bishop_pos = Square::from_algebraic("e2").unwrap();
        let moves = move_gen.generate_bishop_moves(bishop_pos).unwrap();

        assert_eq!(moves.len(), 4);

        let valid_targets = vec![
            Square::from_algebraic("d3").unwrap(),
            Square::from_algebraic("f3").unwrap(),
            Square::from_algebraic("g4").unwrap(),
            Square::from_algebraic("d1").unwrap(),
        ];
        for target in valid_targets {
            let cmove = CMove::new(bishop_pos, target, None);
            assert!(moves.contains(&cmove));
        }
    }

    #[test]
    fn queen_moves_correct() {
        let game = Game::from_fen("k7/8/4P3/1BN5/2Q2B2/2b5/8/K4n2 w - - 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);
        let queen_pos = Square::from_algebraic("c4").unwrap();
        let moves = move_gen.generate_queen_moves(queen_pos).unwrap();

        assert_eq!(moves.len(), 11);

        let valid_targets = vec![
            Square::from_algebraic("d4").unwrap(),
            Square::from_algebraic("e4").unwrap(),
            Square::from_algebraic("d5").unwrap(),
            Square::from_algebraic("b4").unwrap(),
            Square::from_algebraic("a4").unwrap(),
            Square::from_algebraic("b3").unwrap(),
            Square::from_algebraic("a2").unwrap(),
            Square::from_algebraic("c3").unwrap(),
            Square::from_algebraic("d3").unwrap(),
            Square::from_algebraic("e2").unwrap(),
            Square::from_algebraic("f1").unwrap(),
        ];

        for target in valid_targets {
            let cmove = CMove::new(queen_pos, target, None);
            assert!(moves.contains(&cmove));
        }
    }

    #[test]
    fn knight_moves_correct() {
        let game = Game::from_fen("k7/5b2/8/6n1/4P3/5N1p/8/K7 b - - 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);
        let knight_pos = Square::from_algebraic("g5").unwrap();
        let moves = move_gen.generate_knight_moves(knight_pos).unwrap();

        assert_eq!(moves.len(), 4);

        let valid_targets = vec![
            Square::from_algebraic("h7").unwrap(),
            Square::from_algebraic("e6").unwrap(),
            Square::from_algebraic("e4").unwrap(),
            Square::from_algebraic("f3").unwrap(),
        ];

        for target in valid_targets {
            let cmove = CMove::new(knight_pos, target, None);
            assert!(moves.contains(&cmove));
        }
    }

    #[test]
    fn pawn_moves_correct_initial_1() {
        let game = Game::from_fen("8/8/k7/8/5n2/3b2pN/4PPP1/K7 w - - 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);

        let pawn_pos = Square::from_algebraic("e2").unwrap();
        let moves = move_gen.generate_pawn_moves(pawn_pos).unwrap();

        assert_eq!(moves.len(), 3);

        let valid_targets = vec![
            Square::from_algebraic("e3").unwrap(),
            Square::from_algebraic("e4").unwrap(),
            Square::from_algebraic("d3").unwrap(),
        ];

        for target in valid_targets {
            let cmove = CMove::new(pawn_pos, target, None);
            assert!(moves.contains(&cmove));
        }
    }

    #[test]
    fn pawn_moves_correct_initial_2() {
        let game = Game::from_fen("8/8/k7/8/5n2/3b2pN/4PPP1/K7 w - - 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);

        let pawn_pos = Square::from_algebraic("f2").unwrap();
        let moves = move_gen.generate_pawn_moves(pawn_pos).unwrap();

        assert_eq!(moves.len(), 2);

        let valid_targets = vec![
            Square::from_algebraic("g3").unwrap(),
            Square::from_algebraic("f3").unwrap(),
        ];

        for target in valid_targets {
            let cmove = CMove::new(pawn_pos, target, None);
            assert!(moves.contains(&cmove));
        }
    }

    #[test]
    fn pawn_moves_correct_initial_3() {
        let game = Game::from_fen("8/8/k7/8/5n2/3b2pN/4PPP1/K7 w - - 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);

        let pawn_pos = Square::from_algebraic("g2").unwrap();
        let moves = move_gen.generate_pawn_moves(pawn_pos).unwrap();

        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn pawn_moves_correct_initial_black_1() {
        let game = Game::from_fen("k7/3pp3/2rN4/8/8/8/8/K7 b - - 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);

        let pawn_pos = Square::from_algebraic("d7").unwrap();
        let moves = move_gen.generate_pawn_moves(pawn_pos).unwrap();

        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn pawn_moves_correct_initial_black_2() {
        let game = Game::from_fen("k7/3pp3/2rN4/8/8/8/8/K7 b - - 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);

        let pawn_pos = Square::from_algebraic("e7").unwrap();
        let moves = move_gen.generate_pawn_moves(pawn_pos).unwrap();

        assert_eq!(moves.len(), 3);

        let valid_targets = vec![
            Square::from_algebraic("e6").unwrap(),
            Square::from_algebraic("e5").unwrap(),
            Square::from_algebraic("d6").unwrap(),
        ];

        for target in valid_targets {
            let cmove = CMove::new(pawn_pos, target, None);
            assert!(moves.contains(&cmove));
        }
    }

    #[test]
    fn pawn_moves_correct_promotion_black() {
        let game = Game::from_fen("k7/4p3/2r5/8/8/8/3p4/K3N3 b - - 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);

        let pawn_pos = Square::from_algebraic("d2").unwrap();
        let moves = move_gen.generate_pawn_moves(pawn_pos).unwrap();

        assert_eq!(moves.len(), 8);

        let valid_moves = vec![
            CMove::from_long_algebraic("d2d1q").unwrap(),
            CMove::from_long_algebraic("d2d1r").unwrap(),
            CMove::from_long_algebraic("d2d1b").unwrap(),
            CMove::from_long_algebraic("d2d1n").unwrap(),
            CMove::from_long_algebraic("d2e1q").unwrap(),
            CMove::from_long_algebraic("d2e1r").unwrap(),
            CMove::from_long_algebraic("d2e1b").unwrap(),
            CMove::from_long_algebraic("d2e1n").unwrap(),
        ];

        for cmove in valid_moves {
            assert!(moves.contains(&cmove));
        }
    }

    #[test]
    fn pawn_moves_correct_promotion_white() {
        let game = Game::from_fen("k7/3ppP2/8/2r1N3/8/8/8/K7 w - - 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);

        let pawn_pos = Square::from_algebraic("f7").unwrap();
        let moves = move_gen.generate_pawn_moves(pawn_pos).unwrap();

        assert_eq!(moves.len(), 4);

        let valid_moves = vec![
            CMove::from_long_algebraic("f7f8q").unwrap(),
            CMove::from_long_algebraic("f7f8r").unwrap(),
            CMove::from_long_algebraic("f7f8n").unwrap(),
            CMove::from_long_algebraic("f7f8b").unwrap(),
        ];

        for cmove in valid_moves {
            assert!(moves.contains(&cmove));
        }
    }

    #[test]
    fn pawn_moves_correct_en_passant_white() {
        let game = Game::from_fen("k7/3p4/8/2r1pP2/8/8/2N5/K7 w - e6 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);

        let pawn_pos = Square::from_algebraic("f5").unwrap();
        let moves = move_gen.generate_pawn_moves(pawn_pos).unwrap();

        assert_eq!(moves.len(), 2);

        let valid_targets = vec![
            Square::from_algebraic("f6").unwrap(),
            Square::from_algebraic("e6").unwrap(),
        ];

        for target in valid_targets {
            let cmove = CMove::new(pawn_pos, target, None);
            assert!(moves.contains(&cmove));
        }
    }

    #[test]
    fn pawn_moves_correct_no_en_passant_late() {
        let game = Game::from_fen("k7/3p4/2r5/4pP2/8/8/2N5/K7 w - - 0 1").unwrap();
        let move_gen = MoveGenerator::new(&game);

        let pawn_pos = Square::from_algebraic("f5").unwrap();
        let moves = move_gen.generate_pawn_moves(pawn_pos).unwrap();

        assert_eq!(moves.len(), 1);

        let valid_targets = vec![Square::from_algebraic("f6").unwrap()];

        for target in valid_targets {
            let cmove = CMove::new(pawn_pos, target, None);
            assert!(moves.contains(&cmove));
        }
    }

    #[test]
    fn king_moves_correct_castling() {
        let game =
            Game::from_fen("rnbqkbnr/pp4pp/2pp4/4pp2/2BPP3/5N2/PPP2PPP/RNBQK2R w KQkq - 0 1")
                .unwrap();
        let move_gen = MoveGenerator::new(&game);

        let king_pos = Square::from_algebraic("e1").unwrap();
        let moves = move_gen.generate_king_moves(king_pos).unwrap();

        assert_eq!(moves.len(), 4);

        let valid_targets = vec![
            Square::from_algebraic("f1").unwrap(),
            Square::from_algebraic("g1").unwrap(), // Castling
            Square::from_algebraic("e2").unwrap(),
            Square::from_algebraic("d2").unwrap(),
        ];

        for target in valid_targets {
            let cmove = CMove::new(king_pos, target, None);
            assert!(moves.contains(&cmove));
        }
    }

    #[test]
    fn all_moves_correct_default() {
        let game = Game::new_default();
        let move_gen = MoveGenerator::new(&game);
        let moves = move_gen.generate_moves().unwrap();
        assert_eq!(moves.len(), 20);
    }
}
