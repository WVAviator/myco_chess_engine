define_piece!(WhitePawnBitboard);

impl WhitePawnBitboard {
    fn from_fen(fen_board_str: &str) -> Self {
        WhitePawnBitboard(Bitboard::from_fen(fen_board_str, 'P'))
    }

    fn single_advance(&self, occupied: &Bitboard) -> Self {
        WhitePawnBitboard::from((self.0 << 8) & !occupied)
    }

    fn double_advance(&self, occupied: &Bitboard) -> Self {
        let second_rank = self.0 & Bitboard::SECOND_RANK;
        let single_advance = (second_rank << 8) & !occupied;
        WhitePawnBitboard::from((single_advance << 8) & !occupied)
    }

    fn captures(&self, black_pieces: &Bitboard, en_passant: &Bitboard) -> Self {
        let left_diag = (self.0 & !Bitboard::A_FILE) << 7;
        let left_captures = &left_diag & &(black_pieces | en_passant);
        let right_diag = (self.0 & !Bitboard::H_FILE) << 9;
        let right_captures = &right_diag & &(black_pieces | en_passant);

        WhitePawnBitboard::from(left_captures | right_captures)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_single_advance_from_start() {
        let white_pawns =
            WhitePawnBitboard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        let occupied = Bitboard::from(0);
        let single_advance = white_pawns.single_advance(&occupied);
        assert_eq!(single_advance, 0xff0000.into());
        assert_eq!(white_pawns, 0xff00.into());
    }

    #[test]
    fn correct_single_advance_some_blocked() {
        let white_pawns = WhitePawnBitboard::from(0b100001000100001000100000000); // a2, b3, c4, e2, f3
        let occupied = Bitboard::from(0b10000100000000000000000000000000000); // c5, f4
        let expected = WhitePawnBitboard::from(0b10000100010000000000000000); // a3, b4, e3 - c and f files blocked

        let single_advance = white_pawns.single_advance(&occupied);
        assert_eq!(single_advance, expected);
    }

    #[test]
    fn correct_double_advance_some_blocked() {
        let white_pawns = WhitePawnBitboard::from(0b10000000011100000000); // a2, b2, c2, d3
        let occupied = Bitboard::from(0b100000000100000000000000000); // b3, c4
        let expected = WhitePawnBitboard::from(0x1000000); // a4

        let double_advance = white_pawns.double_advance(&occupied);
        assert_eq!(double_advance, expected);
    }
}
