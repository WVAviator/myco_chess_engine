use rand::random;

pub struct MagicHashMap {
    magic_number: u64,
    table: [u64; 4096],
}

pub struct MagicCollisionError;

impl MagicHashMap {
    pub fn new() -> Self {
        let magic_number = Self::generate_magic_number_candidate();

        Self {
            magic_number,
            table: [0; 4096],
        }
    }

    pub fn get(&self, key: u64) -> u64 {
        let hash = self.hash(key);
        self.table[hash]
    }

    pub fn set(&mut self, key: u64, value: u64) -> Result<(), MagicCollisionError> {
        let hash = self.hash(key);
        if self.table[hash] == 0 || self.table[hash] == value {
            self.table[hash] = value;
            Ok(())
        } else {
            Err(MagicCollisionError {})
        }
    }

    fn hash(&self, key: u64) -> usize {
        (self.magic_number.wrapping_mul(key) >> 52) as usize
    }

    fn generate_magic_number_candidate() -> u64 {
        random::<u64>() & random::<u64>() & random::<u64>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generates_random_magic_number() {
        let magic_hashmap1 = MagicHashMap::new();
        let magic_hashmap2 = MagicHashMap::new();
        let magic_hashmap3 = MagicHashMap::new();

        println!(
            "{} {} {}",
            magic_hashmap1.magic_number, magic_hashmap2.magic_number, magic_hashmap3.magic_number
        );

        assert_ne!(magic_hashmap1.magic_number, magic_hashmap2.magic_number);
        assert_ne!(magic_hashmap2.magic_number, magic_hashmap3.magic_number);
    }

    #[test]
    fn generates_hashes_under_4096() {
        let magic_hashmap = MagicHashMap::new();

        assert!(magic_hashmap.hash(2260631642703872) < 4096);
        assert!(magic_hashmap.hash(2251836153661440) < 4096);
        assert!(magic_hashmap.hash(35803103232) < 4096);
        assert!(magic_hashmap.hash(2260632246683648) < 4096);
    }
}
