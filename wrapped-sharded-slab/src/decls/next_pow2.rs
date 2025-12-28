macro_rules! next_pow2 {
    () => {
        pub (crate) const fn next_pow2 (n : usize) -> usize { let pow2 = n . count_ones () == 1 ; let zeros = n . leading_zeros () ; 1 << (WIDTH - zeros as usize - pow2 as usize) }
    };
}

next_pow2!()