use anyhow::bail;

use crate::cgame::constants::{A_FILE, EIGHTH_RANK, FIRST_RANK, H_FILE};

pub fn raycast_rook(rook: u64, blockers: u64) -> Result<u64, anyhow::Error> {
    if rook.count_ones() != 1 {
        bail!("Raycast failed - provided bitboard must contain exactly one rook bit set.");
    }

    let mut result = 0;

    // North
    let mut ray = rook & !EIGHTH_RANK;
    while ray != 0 {
        ray = ray << 8;
        result |= ray;
        ray &= !EIGHTH_RANK;
        ray &= !blockers;
    }

    // South
    let mut ray = rook & !FIRST_RANK;
    while ray != 0 {
        ray = ray >> 8;
        result |= ray;
        ray &= !FIRST_RANK;
        ray &= !blockers;
    }

    // East
    let mut ray = rook & !H_FILE;
    while ray != 0 {
        ray = ray << 1;
        result |= ray;
        ray &= !H_FILE;
        ray &= !blockers;
    }

    // West
    let mut ray = rook & !A_FILE;
    while ray != 0 {
        ray = ray >> 1;
        result |= ray;
        ray &= !A_FILE;
        ray &= !blockers;
    }

    Ok(result)
}

pub fn raycast_bishop(bishop: u64, blockers: u64) -> Result<u64, anyhow::Error> {
    if bishop.count_ones() != 1 {
        bail!("Raycast failed - provided bitboard must contain exactly one bishop bit set.");
    }

    let mut result = 0;

    // Northeast
    let mut ray = bishop & !(EIGHTH_RANK | H_FILE);
    while ray != 0 {
        ray = ray << 9;
        result |= ray;
        ray &= !(EIGHTH_RANK | H_FILE);
        ray &= !blockers;
    }

    // Southeast
    let mut ray = bishop & !(FIRST_RANK | H_FILE);
    while ray != 0 {
        ray = ray >> 7;
        result |= ray;
        ray &= !(FIRST_RANK | H_FILE);
        ray &= !blockers;
    }

    // Southwest
    let mut ray = bishop & !(FIRST_RANK | A_FILE);
    while ray != 0 {
        ray = ray >> 9;
        result |= ray;
        ray &= !(FIRST_RANK | A_FILE);
        ray &= !blockers;
    }

    // Northwest
    let mut ray = bishop & !(EIGHTH_RANK | A_FILE);
    while ray != 0 {
        ray = ray << 7;
        result |= ray;
        ray &= !(EIGHTH_RANK | A_FILE);
        ray &= !blockers;
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn properly_raycasts_rook_blockers() {
        let rook = 0x8000000; // d4
        let blockers = 0x802000000; // b4, d5
        let expected = 0x8f6080808;
        let raycast = raycast_rook(rook, blockers).unwrap();
        assert_eq!(raycast, expected);
    }

    #[test]
    fn properly_raycasts_bishop_blockers() {
        let bishop = 0x8000000; // d4
        let blockers = 0x21000000000; // b6, e5
        let expected = 0x21400142241;
        let raycast = raycast_bishop(bishop, blockers).unwrap();
        assert_eq!(raycast, expected);
    }
}
