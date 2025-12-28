macro_rules! max_dist {
    () => {
        fn max_dist (comp_factor : u8) -> usize { debug_assert ! ((1 ..= 4) . contains (& comp_factor)) ; let v_dist_bits = comp_factor as usize ; 1 << (v_dist_bits + 8) }
    };
}

max_dist!()