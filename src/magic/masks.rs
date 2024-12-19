use anyhow::{anyhow, bail};

use crate::cgame::constants::{A_FILE, EIGHTH_RANK, FIRST_RANK, H_FILE};

pub fn get_rook_mask(rook: u64) -> u64 {
    ROOK_MASKS[rook.trailing_zeros() as usize]
}

pub fn get_bishop_mask(bishop: u64) -> u64 {
    BISHOP_MASKS[bishop.trailing_zeros() as usize]
}

pub const ROOK_MASKS: [u64; 64] = [
    282578800148862,
    565157600297596,
    1130315200595066,
    2260630401190006,
    4521260802379886,
    9042521604759646,
    18085043209519166,
    36170086419038334,
    282578800180736,
    565157600328704,
    1130315200625152,
    2260630401218048,
    4521260802403840,
    9042521604775424,
    18085043209518592,
    36170086419037696,
    282578808340736,
    565157608292864,
    1130315208328192,
    2260630408398848,
    4521260808540160,
    9042521608822784,
    18085043209388032,
    36170086418907136,
    282580897300736,
    565159647117824,
    1130317180306432,
    2260632246683648,
    4521262379438080,
    9042522644946944,
    18085043175964672,
    36170086385483776,
    283115671060736,
    565681586307584,
    1130822006735872,
    2261102847592448,
    4521664529305600,
    9042787892731904,
    18085034619584512,
    36170077829103616,
    420017753620736,
    699298018886144,
    1260057572672512,
    2381576680245248,
    4624614895390720,
    9110691325681664,
    18082844186263552,
    36167887395782656,
    35466950888980736,
    34905104758997504,
    34344362452452352,
    33222877839362048,
    30979908613181440,
    26493970160820224,
    17522093256097792,
    35607136465616896,
    9079539427579068672,
    8935706818303361536,
    8792156787827803136,
    8505056726876686336,
    7930856604974452736,
    6782456361169985536,
    4485655873561051136,
    9115426935197958144,
];

pub const BISHOP_MASKS: [u64; 64] = [
    18049651735527936,
    70506452091904,
    275415828992,
    1075975168,
    38021120,
    8657588224,
    2216338399232,
    567382630219776,
    9024825867763712,
    18049651735527424,
    70506452221952,
    275449643008,
    9733406720,
    2216342585344,
    567382630203392,
    1134765260406784,
    4512412933816832,
    9024825867633664,
    18049651768822272,
    70515108615168,
    2491752130560,
    567383701868544,
    1134765256220672,
    2269530512441344,
    2256206450263040,
    4512412900526080,
    9024834391117824,
    18051867805491712,
    637888545440768,
    1135039602493440,
    2269529440784384,
    4539058881568768,
    1128098963916800,
    2256197927833600,
    4514594912477184,
    9592139778506752,
    19184279556981248,
    2339762086609920,
    4538784537380864,
    9077569074761728,
    562958610993152,
    1125917221986304,
    2814792987328512,
    5629586008178688,
    11259172008099840,
    22518341868716544,
    9007336962655232,
    18014673925310464,
    2216338399232,
    4432676798464,
    11064376819712,
    22137335185408,
    44272556441600,
    87995357200384,
    35253226045952,
    70506452091904,
    567382630219776,
    1134765260406784,
    2832480465846272,
    5667157807464448,
    11333774449049600,
    22526811443298304,
    9024825867763712,
    18049651735527936,
];

// The below code was used to generate the above hardcoded arrays

#[allow(dead_code)]
fn generate_rook_masks() {
    let rook_masks = (0..64)
        .into_iter()
        .map(|sq| calculate_rook_mask(1 << sq).unwrap().to_string())
        .collect::<Vec<String>>();
    println!(
        "pub const ROOK_MASKS: [u64; 64] = [{}];",
        rook_masks.join(", ")
    );
}

#[allow(dead_code)]
fn generate_bishop_masks() {
    let bishop_masks = (0..64)
        .into_iter()
        .map(|sq| calculate_bishop_mask(1 << sq).unwrap().to_string())
        .collect::<Vec<String>>();
    println!(
        "pub const BISHOP_MASKS: [u64; 64] = [{}];",
        bishop_masks.join(", ")
    );
}

#[allow(dead_code)]
fn calculate_rook_mask(square: u64) -> Result<u64, anyhow::Error> {
    let mut result = 0;

    if square.count_ones() != 1 {
        bail!("mask can only be calculated for a single piece");
    }

    // North
    let mut ray = square & !EIGHTH_RANK;
    while ray != 0 {
        ray = ray << 8;
        ray &= !EIGHTH_RANK;
        result |= ray;
    }

    // South
    let mut ray = square & !FIRST_RANK;
    while ray != 0 {
        ray = ray >> 8;
        ray &= !FIRST_RANK;
        result |= ray;
    }

    // East
    let mut ray = square & !H_FILE;
    while ray != 0 {
        ray = ray << 1;
        ray &= !H_FILE;
        result |= ray;
    }

    // West
    let mut ray = square & !A_FILE;
    while ray != 0 {
        ray = ray >> 1;
        ray &= !A_FILE;
        result |= ray;
    }

    Ok(result)
}

#[allow(dead_code)]
fn calculate_bishop_mask(square: u64) -> Result<u64, anyhow::Error> {
    let mut result = 0;

    if square.count_ones() != 1 {
        bail!("mask can only be calculated for a single piece");
    }

    // Northeast
    let mut ray = square & !(EIGHTH_RANK | H_FILE);
    while ray != 0 {
        ray = ray << 9;
        ray &= !(EIGHTH_RANK | H_FILE);
        result |= ray;
    }

    // Southeast
    let mut ray = square & !(FIRST_RANK | H_FILE);
    while ray != 0 {
        ray = ray >> 7;
        ray &= !(FIRST_RANK | H_FILE);
        result |= ray;
    }

    // Southwest
    let mut ray = square & !(FIRST_RANK | A_FILE);
    while ray != 0 {
        ray = ray >> 9;
        ray &= !(FIRST_RANK | A_FILE);
        result |= ray;
    }

    // Northwest
    let mut ray = square & !(EIGHTH_RANK | A_FILE);
    while ray != 0 {
        ray = ray << 7;
        ray &= !(EIGHTH_RANK | A_FILE);
        result |= ray;
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn properly_calculates_rook_mask_center() {
        let rook_square = 0x8000000; // d4
        let mask = calculate_rook_mask(rook_square).unwrap();
        assert_eq!(mask, 0x8080876080800);
    }

    #[test]
    fn gets_correct_rook_mask_center() {
        let rook_square: u64 = 0x8000000; // d4
        let mask = get_rook_mask(rook_square);
        assert_eq!(mask, 0x8080876080800);
    }

    #[test]
    fn properly_calculates_rook_mask_edge() {
        let rook_square = 0x100; // a2
        let mask = calculate_rook_mask(rook_square).unwrap();
        assert_eq!(mask, 0x1010101017e00);
    }

    #[test]
    fn properly_calculates_bishop_mask_center() {
        let bishop_square = 0x8000000; // d4
        let mask = calculate_bishop_mask(bishop_square).unwrap();
        assert_eq!(mask, 0x40221400142200);
    }

    #[test]
    fn properly_calculates_bishop_mask_corner() {
        let bishop_square = 0x2000000000000; // b7
        let mask = calculate_bishop_mask(bishop_square).unwrap();
        assert_eq!(mask, 0x40810204000);
    }

    #[test]
    fn properly_calculates_bishop_mask_edge() {
        let bishop_square = 0x10000; // a3
        let mask = calculate_bishop_mask(bishop_square).unwrap();
        assert_eq!(mask, 0x10080402000200);
    }

    #[ignore = "not needed"]
    #[test]
    fn generates_rook_masks() {
        generate_rook_masks();
    }

    #[ignore = "not needed"]
    #[test]
    fn generates_bishop_masks() {
        generate_bishop_masks();
    }
}
