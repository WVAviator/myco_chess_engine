use std::{fmt, sync::OnceLock};

use anyhow::{bail, Context};
use rand::random;

use crate::moves::{common::PieceType, simple_move::SimpleMove};

use super::{
    constants::{EIGHTH_RANK, FIFTH_RANK, FIRST_RANK, FOURTH_RANK},
    game::Turn,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    pub white_pawns: u64,
    pub white_rooks: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_queens: u64,
    pub white_king: u64,

    pub black_pawns: u64,
    pub black_rooks: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_queens: u64,
    pub black_king: u64,
}

impl Board {
    pub fn new_empty() -> Self {
        Board {
            white_pawns: 0,
            white_rooks: 0,
            white_knights: 0,
            white_bishops: 0,
            white_queens: 0,
            white_king: 0,

            black_pawns: 0,
            black_rooks: 0,
            black_knights: 0,
            black_bishops: 0,
            black_queens: 0,
            black_king: 0,
        }
    }

    pub fn new_default() -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap()
    }

    pub fn iter_mut(&mut self) -> [&mut u64; 12] {
        [
            &mut self.white_pawns,
            &mut self.white_rooks,
            &mut self.white_knights,
            &mut self.white_bishops,
            &mut self.white_queens,
            &mut self.white_king,
            &mut self.black_pawns,
            &mut self.black_rooks,
            &mut self.black_knights,
            &mut self.black_bishops,
            &mut self.black_queens,
            &mut self.black_king,
        ]
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
                    board.white_pawns |= square_bit;
                    file += 1;
                }
                'R' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.white_rooks |= square_bit;
                    file += 1;
                }
                'N' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.white_knights |= square_bit;
                    file += 1;
                }
                'B' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.white_bishops |= square_bit;
                    file += 1;
                }
                'Q' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.white_queens |= square_bit;
                    file += 1;
                }
                'K' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.white_king |= square_bit;
                    file += 1;
                }

                'p' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.black_pawns |= square_bit;
                    file += 1;
                }
                'r' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.black_rooks |= square_bit;
                    file += 1;
                }
                'n' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.black_knights |= square_bit;
                    file += 1;
                }
                'b' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.black_bishops |= square_bit;
                    file += 1;
                }
                'q' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.black_queens |= square_bit;
                    file += 1;
                }
                'k' => {
                    let square_bit = 1 << (rank * 8 + file);
                    board.black_king |= square_bit;
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

        Ok(board)
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for rank in (0..8).rev() {
            let mut empty_count = 0;

            for file in 0..8 {
                let square_index = rank * 8 + file;
                let square_bit = 1 << square_index;

                let piece_char = if self.white_pawns & square_bit != 0 {
                    'P'
                } else if self.white_rooks & square_bit != 0 {
                    'R'
                } else if self.white_knights & square_bit != 0 {
                    'N'
                } else if self.white_bishops & square_bit != 0 {
                    'B'
                } else if self.white_queens & square_bit != 0 {
                    'Q'
                } else if self.white_king & square_bit != 0 {
                    'K'
                } else if self.black_pawns & square_bit != 0 {
                    'p'
                } else if self.black_rooks & square_bit != 0 {
                    'r'
                } else if self.black_knights & square_bit != 0 {
                    'n'
                } else if self.black_bishops & square_bit != 0 {
                    'b'
                } else if self.black_queens & square_bit != 0 {
                    'q'
                } else if self.black_king & square_bit != 0 {
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

    pub fn white_pieces(&self) -> u64 {
        self.white_pawns
            | self.white_rooks
            | self.white_knights
            | self.white_bishops
            | self.white_queens
            | self.white_king
    }

    pub fn black_pieces(&self) -> u64 {
        self.black_pawns
            | self.black_rooks
            | self.black_knights
            | self.black_bishops
            | self.black_queens
            | self.black_king
    }

    pub fn occupied(&self) -> u64 {
        self.white_pieces() | self.black_pieces()
    }

    pub fn empty(&self) -> u64 {
        !self.occupied()
    }

    pub fn pawns(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white_pawns,
            Turn::Black => self.black_pawns,
        }
    }

    pub fn rooks(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white_rooks,
            Turn::Black => self.black_rooks,
        }
    }

    pub fn bishops(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white_bishops,
            Turn::Black => self.black_bishops,
        }
    }

    pub fn knights(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white_knights,
            Turn::Black => self.black_knights,
        }
    }

    pub fn queens(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white_queens,
            Turn::Black => self.black_queens,
        }
    }

    pub fn king(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white_king,
            Turn::Black => self.black_king,
        }
    }

    pub fn all_pieces(&self, turn: &Turn) -> u64 {
        match turn {
            Turn::White => self.white_pieces(),
            Turn::Black => self.black_pieces(),
        }
    }

    pub fn apply_move(&mut self, lmove: &SimpleMove) {
        self.handle_castling(&lmove);
        self.handle_enpassant_takes(&lmove);

        let bitboards = self.iter_mut();

        for bitboard in bitboards {

            // dest & bb will be 0 unless there is a piece at dest to be captured
            // *bitboard ^= lmove.dest & *bitboard;

            // let relevant_bits = *bitboard & (lmove.orig | lmove.dest);
            // let reversed = !relevant_bits & (lmove.orig | lmove.dest);

            // TODO: Find a way to complete the below loop without conditionals


            if lmove.orig & *bitboard != 0 {
                // Moves the piece from orig to dest
                *bitboard ^= lmove.orig | lmove.dest;
            } else if lmove.dest & *bitboard != 0 {
                // Captures pieces located at dest
                *bitboard &= !lmove.dest
            }
        }

        self.handle_promotions(&lmove);
    }

    pub fn handle_castling(&mut self, lmove: &SimpleMove) {
        // Matches the orig king and dest squares to castle patterns (i.e. e1g1)
        // Moves the rook if so, king will be moved later
        match (lmove.orig & (self.black_king | self.white_king)) | lmove.dest {
                0x5000000000000000 => self.black_rooks ^= 0xa000000000000000, // Castle kingside
                0x1400000000000000 => self.black_rooks ^= 0x900000000000000,  // Castle queenside
                0x50 => self.white_rooks ^= 0xa0, // Castle kingside
                0x14 => self.white_rooks ^= 0x9,  // Castle queenside
                _ => {}
        }
    }

    pub fn handle_enpassant_takes(&mut self, lmove: &SimpleMove) {
        if lmove.orig & self.white_pawns & FIFTH_RANK != 0
            && lmove.dest != lmove.orig << 8
            && lmove.dest & self.empty() != 0
        {
            self.black_pawns &= !(lmove.dest >> 8);
        } else if lmove.orig & self.black_pawns & FOURTH_RANK != 0
            && lmove.dest != lmove.orig >> 8
            && lmove.dest & self.empty() != 0
        {
            self.white_pawns &= !(lmove.dest << 8);
        }
    }

    pub fn handle_promotions(&mut self, lmove: &SimpleMove) {
        let black_promotion = self.black_pawns & FIRST_RANK;
        let white_promotion = self.white_pawns & EIGHTH_RANK;
        self.black_pawns ^= black_promotion;
        self.white_pawns ^= white_promotion;

        match lmove.get_promotion() {
            Some(PieceType::Bishop) => {
                self.black_bishops ^= black_promotion;
                self.white_bishops ^= white_promotion;
            }
            Some(PieceType::Rook) => {
                self.black_rooks ^= black_promotion;
                self.white_rooks ^= white_promotion;
            }
            Some(PieceType::Knight) => {
                self.black_knights ^= black_promotion;
                self.white_knights ^= white_promotion;
            }
            Some(PieceType::Queen) => {
                self.black_queens ^= black_promotion;
                self.white_queens ^= white_promotion;
            }
            _ => {}
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square_index = rank * 8 + file;
                let square_bit = 1 << square_index;

                let piece_char = if self.white_pawns & square_bit != 0 {
                    '♙'
                } else if self.white_rooks & square_bit != 0 {
                    '♖'
                } else if self.white_knights & square_bit != 0 {
                    '♘'
                } else if self.white_bishops & square_bit != 0 {
                    '♗'
                } else if self.white_queens & square_bit != 0 {
                    '♕'
                } else if self.white_king & square_bit != 0 {
                    '♔'
                } else if self.black_pawns & square_bit != 0 {
                    '♟'
                } else if self.black_rooks & square_bit != 0 {
                    '♜'
                } else if self.black_knights & square_bit != 0 {
                    '♞'
                } else if self.black_bishops & square_bit != 0 {
                    '♝'
                } else if self.black_queens & square_bit != 0 {
                    '♛'
                } else if self.black_king & square_bit != 0 {
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

        assert_eq!(board.white_pawns, 0xff00);
        assert_eq!(board.black_pawns, 0xff000000000000);
        assert_eq!(board.black_rooks, 0x8100000000000000);
        assert_eq!(board.white_bishops, 0x24);
        assert_eq!(board.black_queens, 0x800000000000000);
        assert_eq!(board.white_king, 0x10);
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
