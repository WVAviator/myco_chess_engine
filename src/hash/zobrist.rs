use std::{
    collections::HashSet,
    fs,
    io::Write,
    simd::{num::SimdUint, Simd},
};

use rand::random;

use crate::game::{
    castling_rights::CastlingRights,
    game::{Game, Turn},
};

include!("random_hash_values.rs");

const ZERO: Simd<u64, 8> = Simd::from_array([0, 0, 0, 0, 0, 0, 0, 0]);
const ONE: Simd<u64, 8> = Simd::from_array([1, 1, 1, 1, 1, 1, 1, 1]);
const MASK: Simd<u64, 8> = Simd::from_array([1, 1, 1, 1, 1, 1, 0, 0]);

const WHITE_OFFSETS: Simd<u64, 8> = Simd::from_array([0, 65, 130, 195, 260, 325, 0, 0]);
const BLACK_OFFSETS: Simd<u64, 8> = Simd::from_array([390, 455, 520, 585, 650, 715, 0, 0]);

pub trait ZobristHash {
    fn zobrist(&self) -> u64;
}

impl ZobristHash for Game {
    fn zobrist(&self) -> u64 {
        let mut hash: u64 = 0;

        let mut remaining_white = self.board.white * MASK;

        while remaining_white != ZERO {
            let index = remaining_white.trailing_zeros() + WHITE_OFFSETS;

            let result = Simd::gather_or(&RANDOM_HASH_VALUES, index.cast(), ZERO.cast());
            hash ^= result.reduce_xor();

            remaining_white &= remaining_white - ONE;
        }

        let mut remaining_black = self.board.black * MASK;

        while remaining_black != ZERO {
            let index = remaining_black.trailing_zeros() + BLACK_OFFSETS;

            let result = Simd::gather_or(&RANDOM_HASH_VALUES, index.cast(), ZERO.cast());
            hash ^= result.reduce_xor();

            remaining_black &= remaining_black - ONE;
        }

        let castling_values = [
            (CastlingRights::WHITE_KINGSIDE, 781),
            (CastlingRights::WHITE_QUEENSIDE, 782),
            (CastlingRights::BLACK_KINGSIDE, 783),
            (CastlingRights::BLACK_QUEENSIDE, 784),
        ];

        for (right, index) in castling_values.iter() {
            if self.castling_rights.is_set(*right) {
                hash ^= RANDOM_HASH_VALUES[*index];
            }
        }

        if self.en_passant != 0 {
            let en_passant_file = self.en_passant.trailing_zeros() as usize % 8;
            hash ^= RANDOM_HASH_VALUES[785 + en_passant_file];
        }

        if self.turn == Turn::Black {
            hash ^= RANDOM_HASH_VALUES[793];
        }

        hash
    }
}

#[allow(dead_code)]
pub fn generate_hash_values(n: usize) {
    let file_path = "./src/hash/random_hash_values.rs";

    let mut values = vec![1, 1];
    while values.len() != values.iter().copied().collect::<HashSet<u64>>().len() {
        values = (0..n).map(|_| random::<u64>()).collect::<Vec<u64>>();
    }

    let mut i = 65;
    while i < n {
        values[i] = 0;
        i += 65;
    }

    let values = values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    let content = format!("const RANDOM_HASH_VALUES: [u64; {}] = [{}];", n, values);

    let mut file = fs::File::create(file_path).expect("Could not create file.");
    file.write_all(content.as_bytes())
        .expect("Could not write to file.");
}

#[cfg(test)]
mod test {

    use std::collections::HashSet;

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

    #[test]
    fn all_hash_values_unique() {
        let hash_values_set = RANDOM_HASH_VALUES.iter().copied().collect::<HashSet<u64>>();
        // Minus the twelve zeroes (eleven redundant) for the 64th index of each board
        assert_eq!(RANDOM_HASH_VALUES.len() - 11, hash_values_set.len());
    }
}
