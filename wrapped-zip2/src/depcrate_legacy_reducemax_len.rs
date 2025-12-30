// Generated macro for max_len (function)
macro_rules! Depcrate_legacy_reducemax_len {
() => {
// Module: crate::legacy::reduce
// Provides: {"max_len"}
// Dependencies: {}
fn max_len (comp_factor : u8) -> usize { debug_assert ! ((1 ..= 4) . contains (& comp_factor)) ; let v_len_bits = (8 - comp_factor) as usize ; ((1 << v_len_bits) - 1) + u8 :: MAX as usize + 3 }
};
}
