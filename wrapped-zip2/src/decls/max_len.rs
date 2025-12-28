macro_rules! max_len {
    () => {
        fn max_len (comp_factor : u8) -> usize { debug_assert ! ((1 ..= 4) . contains (& comp_factor)) ; let v_len_bits = (8 - comp_factor) as usize ; ((1 << v_len_bits) - 1) + u8 :: MAX as usize + 3 }
    };
}

max_len!()