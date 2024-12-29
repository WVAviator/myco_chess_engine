use std::simd::{num::SimdUint, Simd};

const ZERO: Simd<u64, 8> = Simd::from_array([0, 0, 0, 0, 0, 0, 0, 0]);
const ONE: Simd<u64, 8> = Simd::from_array([1, 1, 1, 1, 1, 1, 1, 1]);

pub trait SimdCountOnes {
    fn count_ones(self) -> Self;
}

impl SimdCountOnes for Simd<u64, 8> {
    fn count_ones(self) -> Self {
        unsafe { core::intrinsics::simd::simd_ctpop(self) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simd_count_ones_works() {
        let simd = Simd::from_array([1, 1, 2, 2, 3, 3, 4, 4]);
        let count_ones_simd = simd.count_ones();
        let result_array = count_ones_simd.to_array();

        assert_eq!(result_array, [1, 1, 1, 1, 2, 2, 1, 1]);
    }
}
