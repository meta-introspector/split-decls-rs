// Generated macro for max_dist (function)
macro_rules! Depcrate_legacy_reducemax_dist {
() => {
// Module: crate::legacy::reduce
// Provides: {"max_dist"}
// Dependencies: {}
fn max_dist (comp_factor : u8) -> usize { debug_assert ! ((1 ..= 4) . contains (& comp_factor)) ; let v_dist_bits = comp_factor as usize ; 1 << (v_dist_bits + 8) }
};
}
