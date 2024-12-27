use std::fmt;

use anyhow::{bail, Context};

use crate::moves::simple_move::SimpleMove;

use super::{
    constants::{EIGHTH_RANK, FIFTH_RANK, FIRST_RANK, FOURTH_RANK},
    game::Turn,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    pub white: [u64; 6], // pawns, rooks, knights, bishops, queens, king
    pub black: [u64; 6],
    pub all: u64,
    pub white_pieces: u64,
    pub black_pieces: u64,
}

impl Board {
    pub fn new_empty() -> Self {
        Board {
            white: [0; 6],
            black: [0; 6],
            all: 0,
            white_pieces: 0,
            black_pieces: 0,
        }
    }

    pub fn new_default() -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap()
    }

    pub fn from_fen(fen_board_str: &str) -> Result<Self, anyhow::Error> {
        let mut board = Board::new_empty();

        let mut rank = 7;
        let mut file = 0;

        for c in fen_board_str.chars() {
            if file > 8 {
                bail!(
                    "Invalid number of squares in rank {} of FEN string: {}",
                    rank,
                    fen_board_str
                );
            }
            match c {
                'P' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.white[0] |= square_bit;
                    file += 1;
                }
                'R' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.white[1] |= square_bit;
                    file += 1;
                }
                'N' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.white[2] |= square_bit;
                    file += 1;
                }
                'B' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.white[3] |= square_bit;
                    file += 1;
                }
                'Q' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.white[4] |= square_bit;
                    file += 1;
                }
                'K' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.white[5] |= square_bit;
                    file += 1;
                }

                'p' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.black[0] |= square_bit;
                    file += 1;
                }
                'r' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.black[1] |= square_bit;
                    file += 1;
                }
                'n' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.black[2] |= square_bit;
                    file += 1;
                }
                'b' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.black[3] |= square_bit;
                    file += 1;
                }
                'q' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.black[4] |= square_bit;
                    file += 1;
                }
                'k' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.black[5] |= square_bit;
                    file += 1;
                }

                '1'..='8' => {
                    file += c.to_digit(10).context("Unable to convert digit.")? as usize;
                }

                '/' => {
                    rank -= 1;
                    file = 0;
                }

                ch => bail!(
                    "Unrecognized character '{}' in FEN string: {}",
                    ch,
                    fen_board_str
                ),
            }
        }

        board.all = board.get_all();
        board.white_pieces = board.get_white_pieces();
        board.black_pieces = board.get_black_pieces();

        Ok(board)
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for rank in (0..8).rev() {
            let mut empty_count = 0;

            for file in 0..8 {
                let square_index = rank * 8 + file;
                let square_bit = 1 << square_index;

                let piece_char = if self.white[0] & square_bit != 0 {
                    'P'
                } else if self.white[1] & square_bit != 0 {
                    'R'
                } else if self.white[2] & square_bit != 0 {
                    'N'
                } else if self.white[3] & square_bit != 0 {
                    'B'
                } else if self.white[4] & square_bit != 0 {
                    'Q'
                } else if self.white[5] & square_bit != 0 {
                    'K'
                } else if self.black[0] & square_bit != 0 {
                    'p'
                } else if self.black[1] & square_bit != 0 {
                    'r'
                } else if self.black[2] & square_bit != 0 {
                    'n'
                } else if self.black[3] & square_bit != 0 {
                    'b'
                } else if self.black[4] & square_bit != 0 {
                    'q'
                } else if self.black[5] & square_bit != 0 {
                    'k'
                } else {
                    empty_count += 1;
                    continue;
                };

                if empty_count > 0 {
                    fen.push_str(&empty_count.to_string());
                    empty_count = 0;
                }

                fen.push(piece_char);
            }

            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }

            if rank > 0 {
                fen.push('/');
            }
        }

        fen
    }

    fn get_white_pieces(&self) -> u64 {
        self.white[0]
            | self.white[1]
            | self.white[2]
            | self.white[3]
            | self.white[4]
            | self.white[5]
    }

    fn get_black_pieces(&self) -> u64 {
        self.black[0]
            | self.black[1]
            | self.black[2]
            | self.black[3]
            | self.black[4]
            | self.black[5]
    }

    fn get_all(&self) -> u64 {
        self.get_white_pieces() | self.get_black_pieces()
    }

    pub fn empty(&self) -> u64 {
        !self.all
    }

    pub fn pawns(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white[0],
            Turn::Black => self.black[0],
        }
    }

    pub fn rooks(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white[1],
            Turn::Black => self.black[1],
        }
    }

    pub fn bishops(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white[3],
            Turn::Black => self.black[3],
        }
    }

    pub fn knights(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white[2],
            Turn::Black => self.black[2],
        }
    }

    pub fn queens(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white[4],
            Turn::Black => self.black[4],
        }
    }

    pub fn king(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white[5],
            Turn::Black => self.black[5],
        }
    }

    pub fn all_pieces(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.get_white_pieces(),
            Turn::Black => self.get_black_pieces(),
        }
    }

    pub fn apply_move(&mut self, lmove: &SimpleMove) {
        self.handle_castling(&lmove);

        if (lmove.orig & self.white[0] & FIFTH_RANK) | (lmove.orig & self.black[0] & FOURTH_RANK)
            != 0
        {
            self.black[0] ^= ((((lmove.orig & self.white[0] & FIFTH_RANK) << 7)
                & (lmove.dest & self.empty()))
                | ((lmove.orig & self.white[0] & FIFTH_RANK) << 9) & (lmove.dest & self.empty()))
                >> 8;
            self.white[0] ^= ((((lmove.orig & self.black[0] & FOURTH_RANK) >> 9)
                & (lmove.dest & self.empty()))
                | (((lmove.orig & self.black[0] & FOURTH_RANK) >> 7)
                    & (lmove.dest & self.empty())))
                << 8;
        }

        let move_shift: u32 =
            (64 + (lmove.orig.trailing_zeros() as i32 - lmove.dest.trailing_zeros() as i32)) as u32;

        if (lmove.orig & self.white_pieces) | (lmove.dest & self.white_pieces) != 0 {
            for bitboard in self.white.iter_mut() {
                // dest & bb will be 0 unless there is a piece at dest to be captured
                *bitboard &= !lmove.dest;
                // does nothing if orig & bb is 0, otherwise xors with both bits
                *bitboard ^=
                    (lmove.orig & *bitboard) | (lmove.orig & *bitboard).rotate_right(move_shift);
            }
        }

        if (lmove.orig & self.black_pieces) | (lmove.dest & self.black_pieces) != 0 {
            for bitboard in self.black.iter_mut() {
                // dest & bb will be 0 unless there is a piece at dest to be captured
                *bitboard &= !lmove.dest;
                // does nothing if orig & bb is 0, otherwise xors with both bits
                *bitboard ^=
                    (lmove.orig & *bitboard) | (lmove.orig & *bitboard).rotate_right(move_shift);
            }
        }

        self.handle_promotions(&lmove);

        self.all = self.get_all();
        self.white_pieces = self.get_white_pieces();
        self.black_pieces = self.get_black_pieces();
    }

    pub fn handle_castling(&mut self, lmove: &SimpleMove) {
        // Matches the orig king and dest squares to castle patterns (i.e. e1g1)
        // Moves the rook if so, king will be moved later
        match (lmove.orig & (self.black[5] | self.white[5])) | lmove.dest {
            0x5000000000000000 => self.black[1] ^= 0xa000000000000000, // Castle kingside
            0x1400000000000000 => self.black[1] ^= 0x900000000000000,  // Castle queenside
            0x50 => self.white[1] ^= 0xa0,                             // Castle kingside
            0x14 => self.white[1] ^= 0x9,                              // Castle queenside
            _ => {}
        }
    }

    pub fn handle_promotions(&mut self, lmove: &SimpleMove) {
        let black_promotion = self.black[0] & FIRST_RANK;
        let white_promotion = self.white[0] & EIGHTH_RANK;
        self.black[0] ^= black_promotion;
        self.white[0] ^= white_promotion;
        self.black[lmove.promotion] ^= black_promotion;
        self.white[lmove.promotion] ^= white_promotion;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square_index = rank * 8 + file;
                let square_bit = 1 << square_index;

                let piece_char = if self.white[0] & square_bit != 0 {
                    '♙'
                } else if self.white[1] & square_bit != 0 {
                    '♖'
                } else if self.white[2] & square_bit != 0 {
                    '♘'
                } else if self.white[3] & square_bit != 0 {
                    '♗'
                } else if self.white[4] & square_bit != 0 {
                    '♕'
                } else if self.white[5] & square_bit != 0 {
                    '♔'
                } else if self.black[0] & square_bit != 0 {
                    '♟'
                } else if self.black[1] & square_bit != 0 {
                    '♜'
                } else if self.black[2] & square_bit != 0 {
                    '♞'
                } else if self.black[3] & square_bit != 0 {
                    '♝'
                } else if self.black[4] & square_bit != 0 {
                    '♛'
                } else if self.black[5] & square_bit != 0 {
                    '♚'
                } else {
                    '·'
                };

                write!(f, "{}", piece_char)?;
            }
            writeln!(f)?; // Start a new line after each rank
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reads_fen_starting_position() {
        let board = Board::new_default();

        assert_eq!(board.white[0], 0xff00);
        assert_eq!(board.black[0], 0xff000000000000);
        assert_eq!(board.black[1], 0x8100000000000000);
        assert_eq!(board.white[3], 0x24);
        assert_eq!(board.black[4], 0x800000000000000);
        assert_eq!(board.white[5], 0x10);
    }

    #[test]
    fn parses_starting_position_to_fen() {
        let board = Board::new_default();
        let fen = board.to_fen();
        assert_eq!(fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }

    #[test]
    fn test_apply_regular_move() {
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR").unwrap();
        let lmove = SimpleMove::from_algebraic("e7e5").unwrap();
        let expected_board =
            Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR").unwrap();
        board.apply_move(&lmove);
        assert_eq!(board, expected_board);
    }

    #[test]
    fn test_apply_take_move() {
        let mut board =
            Board::from_fen("rnbqkbnr/ppp2ppp/3p4/4p3/4PP2/8/PPPP2PP/RNBQKBNR").unwrap();
        let lmove = SimpleMove::from_algebraic("f4e5").unwrap();
        let expected_board =
            Board::from_fen("rnbqkbnr/ppp2ppp/3p4/4P3/4P3/8/PPPP2PP/RNBQKBNR").unwrap();
        board.apply_move(&lmove);
        assert_eq!(board, expected_board);
    }

    #[test]
    fn test_apply_take_enpassant() {
        let mut board = Board::from_fen("rnbqkbnr/ppp3pp/3p4/4Pp2/4P3/8/PPPP2PP/RNBQKBNR").unwrap();
        let lmove = SimpleMove::from_algebraic("e5f6").unwrap();
        let expected_board =
            Board::from_fen("rnbqkbnr/ppp3pp/3p1P2/8/4P3/8/PPPP2PP/RNBQKBNR").unwrap();
        board.apply_move(&lmove);
        assert_eq!(board, expected_board);
    }

    #[test]
    fn test_apply_castles() {
        let mut board =
            Board::from_fen("rnbqk2r/ppp1ppbp/3p1np1/8/3PP3/2PB1N2/PP3PPP/RNBQK2R").unwrap();
        let lmove = SimpleMove::from_algebraic("e8g8").unwrap();
        let expected_board =
            Board::from_fen("rnbq1rk1/ppp1ppbp/3p1np1/8/3PP3/2PB1N2/PP3PPP/RNBQK2R").unwrap();
        board.apply_move(&lmove);
        assert_eq!(board, expected_board);
    }

    #[test]
    fn test_apply_promotion() {
        let mut board =
            Board::from_fen("rnb2rk1/pp1Pqpbp/2p1pnp1/8/3P4/2PB1N2/PP3PPP/RNBQK2R").unwrap();
        let lmove = SimpleMove::from_algebraic("d7d8q").unwrap();
        let expected_board =
            Board::from_fen("rnbQ1rk1/pp2qpbp/2p1pnp1/8/3P4/2PB1N2/PP3PPP/RNBQK2R").unwrap();
        board.apply_move(&lmove);
        assert_eq!(board, expected_board);
    }
}
