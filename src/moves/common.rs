#[derive(Debug, PartialEq, Clone, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub fn algebraic_to_u64(square: &str) -> u64 {
    if square.len() != 2 {
        panic!("Invalid square format: {}", square);
    }

    let chars: Vec<char> = square.chars().collect();
    let file = chars[0];
    let rank = chars[1];

    if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
        panic!("Invalid square coordinates: {}", square);
    }

    let file_index = (file as u8 - b'a') as u64;
    let rank_index = (rank as u8 - b'1') as u64;

    let square_bit = 1u64 << (rank_index * 8 + file_index);

    square_bit
}

pub fn u64_to_algebraic(square: u64) -> String {
    if square == 0 || square.count_ones() != 1 {
        panic!(
            "Invalid square for algebraic conversion: {}. Must have only a single bit set.",
            square
        );
    }

    let position = square.trailing_zeros() as u64;
    let rank = position / 8;
    let file = position % 8;

    let file_char = (b'a' + file as u8) as char;
    let rank_char = (b'1' + rank as u8) as char;

    format!("{}{}", file_char, rank_char)
}
