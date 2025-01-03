pub struct BitIndexIterator(u64);

impl BitIndexIterator {
    fn new(value: u64) -> Self {
        BitIndexIterator(value)
    }
}

impl Iterator for BitIndexIterator {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit_index = self.0.trailing_zeros() as usize;
        self.0 &= self.0 - 1;

        Some(bit_index)
    }
}

pub trait BitIndexIterable {
    fn bit_indexes(self) -> BitIndexIterator;
}

impl BitIndexIterable for u64 {
    fn bit_indexes(self) -> BitIndexIterator {
        BitIndexIterator::new(self)
    }
}

pub struct BitIterator(u64);

impl BitIterator {
    fn new(value: u64) -> Self {
        BitIterator(value)
    }
}

impl Iterator for BitIterator {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let lsb = self.0 & (!self.0 + 1);
        self.0 &= self.0 - 1;

        Some(lsb)
    }
}

pub trait BitIterable {
    fn bits(self) -> BitIterator;
}

impl BitIterable for u64 {
    fn bits(self) -> BitIterator {
        BitIterator::new(self)
    }
}
