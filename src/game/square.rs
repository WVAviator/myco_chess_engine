use std::ops::Deref;

use anyhow::bail;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square(usize);

impl Square {
    pub fn to_algebraic(&self) -> String {
        let file = (self.0 % 8) as u8 + b'a';
        let rank = 7 - (self.0 / 8) as u8 + b'1';
        format!("{}{}", file as char, rank as char)
    }

    pub fn from_algebraic(algebraic: &str) -> Result<Self, anyhow::Error> {
        if algebraic.len() != 2 {
            bail!("Invalid algebraic notation for square: {}", algebraic);
        }
        let bytes = algebraic.as_bytes();
        let file = bytes[0];
        let rank = bytes[1];

        if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
            bail!(
                "Invalid algebraic characters in square notation: {}",
                algebraic
            );
        }

        let file_index = file - b'a';
        let rank_index = 7 - (rank - b'1');

        Ok(Square((rank_index * 8 + file_index) as usize))
    }

    pub fn from_position(row: u8, col: u8) -> Result<Self, anyhow::Error> {
        if row > 7 || col > 7 {
            bail!(
                "Attempted to create invalid square at position: {}, {}",
                row,
                col
            );
        }
        Ok(Square(((7 - row) * 8 + col) as usize))
    }

    pub fn from_rank_file(rank: u8, file: char) -> Result<Self, anyhow::Error> {
        let algebraic = format!("{}{}", file, rank);
        Self::from_algebraic(&algebraic)
    }

    pub fn get_row(&self) -> u8 {
        7 - (self.0 / 8) as u8
    }

    pub fn get_col(&self) -> u8 {
        (self.0 % 8) as u8
    }

    pub fn get_rank(&self) -> u8 {
        7 - (self.0 / 8) as u8 + 1
    }

    pub fn get_file(&self) -> char {
        ((self.0 % 8) as u8 + b'a') as char
    }
}

impl Deref for Square {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn square_from_algebraic() {
        assert_eq!(Square::from_algebraic("a8").unwrap().0, 0);
        assert_eq!(Square::from_algebraic("h8").unwrap().0, 7);
        assert_eq!(Square::from_algebraic("a7").unwrap().0, 8);
        assert_eq!(Square::from_algebraic("a1").unwrap().0, 56);
        assert_eq!(Square::from_algebraic("h1").unwrap().0, 63);
        assert_eq!(Square::from_algebraic("d5").unwrap().0, 27);
    }

    #[test]
    fn square_to_algebraic() {
        assert_eq!(Square(0).to_algebraic(), "a8");
        assert_eq!(Square(7).to_algebraic(), "h8");
        assert_eq!(Square(8).to_algebraic(), "a7");
        assert_eq!(Square(56).to_algebraic(), "a1");
        assert_eq!(Square(63).to_algebraic(), "h1");
        assert_eq!(Square(27).to_algebraic(), "d5");
    }

    #[test]
    fn square_invalid_algebraic_notation() {
        assert!(Square::from_algebraic("z1").is_err()); // Invalid file
        assert!(Square::from_algebraic("a9").is_err()); // Invalid rank
        assert!(Square::from_algebraic("a").is_err()); // Too short
        assert!(Square::from_algebraic("a11").is_err()); // Too long
    }

    #[test]
    fn test_round_trip() {
        let original = Square(36);
        let algebraic = original.clone().to_algebraic();
        let converted = Square::from_algebraic(&algebraic).unwrap();
        assert_eq!(original.0, converted.0);
    }

    #[test]
    fn deref_to_usize() {
        let list = vec![1, 2, 3, 4, 5];
        let square = Square(2);
        assert_eq!(list[*square], 3);
    }

    #[test]
    fn row_col_correct() {
        let original = Square::from_position(4, 6).unwrap();
        assert_eq!(original.get_row(), 4);
        assert_eq!(original.get_col(), 6);
    }

    #[test]
    fn rank_file_correct() {
        let square = Square(8);
        assert_eq!(square.get_rank(), 7);
        assert_eq!(square.get_file(), 'a');
    }
}
