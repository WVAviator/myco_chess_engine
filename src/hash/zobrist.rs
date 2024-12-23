use std::{fs, io::Write, path::Path};

use rand::random;

use crate::cgame::{
    castling_rights::CastlingRights,
    game::{Game, Turn},
};

include!("random_hash_values.rs");

pub trait ZobristHash {
    fn zobrist(&self) -> u64;
}

impl ZobristHash for Game {
    fn zobrist(&self) -> u64 {
        let mut hash: u64 = 0;

        let piece_bitboards = [
            (self.board.white_pawns, 0),
            (self.board.white_rooks, 1),
            (self.board.white_knights, 2),
            (self.board.white_bishops, 3),
            (self.board.white_queens, 4),
            (self.board.white_king, 5),
            (self.board.black_pawns, 6),
            (self.board.black_rooks, 7),
            (self.board.black_knights, 8),
            (self.board.black_bishops, 9),
            (self.board.black_queens, 10),
            (self.board.black_king, 11),
        ];

        for (bitboard, piece_index) in piece_bitboards.iter() {
            let mut bb = *bitboard;
            while bb != 0 {
                let square = bb.trailing_zeros() as usize;
                hash ^= RANDOM_HASH_VALUES[square * 12 + piece_index];
                bb &= bb - 1;
            }
        }

        let castling_values = [
            (CastlingRights::WHITE_KINGSIDE, 768),
            (CastlingRights::WHITE_QUEENSIDE, 769),
            (CastlingRights::BLACK_KINGSIDE, 770),
            (CastlingRights::BLACK_QUEENSIDE, 771),
        ];

        for (right, index) in castling_values.iter() {
            if self.castling_rights.is_set(*right) {
                hash ^= RANDOM_HASH_VALUES[*index];
            }
        }

        if self.en_passant != 0 {
            let en_passant_file = self.en_passant.trailing_zeros() as usize % 8;
            hash ^= RANDOM_HASH_VALUES[772 + en_passant_file];
        }

        if self.turn == Turn::Black {
            hash ^= RANDOM_HASH_VALUES[780];
        }

        hash
    }
}

#[allow(dead_code)]
pub fn generate_hashes() {
    let file_path = "./src/hash/random_hash_values.rs";
    match Path::new(file_path).exists() {
            true => panic!("Cannot overwrite existing hash values! Delete the old file first. Warning: Any existing database items that rely on zobrist hashes will become useless and will need to be rehashed."),
            false => {
                let values = (0..781)
                    .into_iter()
                    .map(|_| random::<u64>().to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let content = format!("const RANDOM_HASH_VALUES: [u64; 781] = [{}];", values);

                let mut file = fs::File::create(file_path).expect("Could not create file.");
                file.write_all(content.as_bytes())
                    .expect("Could not write to file.");
            }
        }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_zobrist_similar_positions() {
        let game1 =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let game2 =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1").unwrap();

        let hash1 = game1.zobrist();
        let hash2 = game2.zobrist();

        assert_ne!(
            hash1, hash2,
            "Hashes for different positions should not be equal"
        );
    }

    #[test]
    fn test_zobrist_identical_positions() {
        let game1 =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let game2 =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        let hash1 = game1.zobrist();
        let hash2 = game2.zobrist();

        assert_eq!(
            hash1, hash2,
            "Hashes for identical positions should be equal"
        );
    }

    #[test]
    fn test_zobrist_turn_difference() {
        let game1 =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let game2 =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();

        let hash1 = game1.zobrist();
        let hash2 = game2.zobrist();

        assert_ne!(
            hash1, hash2,
            "Hashes for positions differing only in turn should not be equal"
        );
    }

    #[test]
    fn test_zobrist_castling_rights_difference() {
        let game1 =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let game2 =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1").unwrap();

        let hash1 = game1.zobrist();
        let hash2 = game2.zobrist();

        assert_ne!(
            hash1, hash2,
            "Hashes for positions differing in castling rights should not be equal"
        );
    }

    #[test]
    fn test_zobrist_en_passant_difference() {
        let game1 =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let game2 =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1").unwrap();

        let hash1 = game1.zobrist();
        let hash2 = game2.zobrist();

        assert_ne!(
            hash1, hash2,
            "Hashes for positions differing in en passant target should not be equal"
        );
    }
}
