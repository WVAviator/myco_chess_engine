pub fn calculate_subsets(mask: u64) -> Vec<u64> {
    let mut subsets = Vec::new();
    let mut current_subset: u64 = 0;
    loop {
        current_subset = current_subset.wrapping_sub(mask) & mask;
        subsets.push(current_subset);
        if current_subset == 0 {
            break;
        }
    }
    subsets
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gets_all_subsets_contiguous_bits() {
        let mask = 0b0000_0111;
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let subsets = calculate_subsets(mask);

        assert_eq!(subsets.len(), expected.len());

        expected
            .iter()
            .for_each(|value| assert!(subsets.contains(value)));
    }

    #[test]
    fn gets_all_subsets_noncontiguous_bits() {
        let mask = 0b1000_0011;
        let expected = vec![0, 1, 2, 3, 128, 129, 130, 131];
        let subsets = calculate_subsets(mask);

        assert_eq!(subsets.len(), expected.len());

        expected
            .iter()
            .for_each(|value| assert!(subsets.contains(value)));
    }
}
