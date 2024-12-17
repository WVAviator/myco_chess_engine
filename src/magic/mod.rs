use anyhow::bail;
use hashmap::MagicHashMap;
use masks::{get_bishop_mask, get_rook_mask};
use raycast::{raycast_bishop, raycast_rook};
use subsets::calculate_subsets;

use crate::cgame::moves::u64_to_algebraic;

mod masks;
mod raycast;
mod subsets;

pub mod hashmap;

pub fn generate_rook_magic_hashmap(rook: u64) -> Result<MagicHashMap, anyhow::Error> {
    let mask = get_rook_mask(rook)?;
    let blocker_subsets = calculate_subsets(mask);

    let movesets: Vec<u64> = blocker_subsets
        .iter()
        .map(|blockers| raycast_rook(rook, *blockers).unwrap())
        .collect();

    for i in 0..10000 {
        let mut failure = false;
        let mut magic_hashmap = MagicHashMap::new();
        for index in 0..blocker_subsets.len() {
            let blockers = blocker_subsets[index];
            let moveset = movesets[index];
            if let Err(_) = magic_hashmap.set(blockers, moveset) {
                failure = true;
                break;
            }
        }
        if failure == false {
            println!(
                "Generated rook magic hashmap for square {} after {} iterations.",
                u64_to_algebraic(rook).unwrap(),
                i
            );
            return Ok(magic_hashmap);
        }
    }

    bail!("failed to generate a magic number after 10000 attempts");
}

pub fn generate_bishop_magic_hashmap(bishop: u64) -> Result<MagicHashMap, anyhow::Error> {
    let mask = get_bishop_mask(bishop)?;
    let blocker_subsets = calculate_subsets(mask);

    let movesets: Vec<u64> = blocker_subsets
        .iter()
        .map(|blockers| raycast_bishop(bishop, *blockers).unwrap())
        .collect();

    for i in 0..10000 {
        let mut failure = false;
        let mut magic_hashmap = MagicHashMap::new();
        for index in 0..blocker_subsets.len() {
            let blockers = blocker_subsets[index];
            let moveset = movesets[index];
            if let Err(_) = magic_hashmap.set(blockers, moveset) {
                failure = true;
                break;
            }
        }
        if failure == false {
            println!(
                "Generated rook magic hashmap for square {} after {} iterations.",
                u64_to_algebraic(bishop).unwrap(),
                i
            );
            return Ok(magic_hashmap);
        }
    }

    bail!("failed to generate a magic number after 10000 attempts");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generates_rook_magic_hashmap() {
        let magic_hashmap = generate_rook_magic_hashmap(0x8000000).unwrap(); // d4

        // validate some various combinations to make sure they work
        assert_eq!(magic_hashmap.get(8796361457664), 8830839162888);
        assert_eq!(magic_hashmap.get(2251800920983552), 2260632246683648);
        assert_eq!(magic_hashmap.get(2251801458378752), 2260631172939776);
    }

    #[test]
    fn generates_bishop_magic_hashmap() {
        let magic_hashmap = generate_bishop_magic_hashmap(0x8000000).unwrap(); // d4

        // validate some various combinations to make sure they work
        assert_eq!(magic_hashmap.get(18051850624303104), 2284923920961);
        assert_eq!(magic_hashmap.get(85899354624), 85900665344);
        assert_eq!(magic_hashmap.get(18014398509481984), 18333342782202433);
    }
}
