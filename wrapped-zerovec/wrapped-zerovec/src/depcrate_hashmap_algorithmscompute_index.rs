// Generated macro for compute_index (function)
macro_rules! Depcrate_hashmap_algorithmscompute_index {
() => {
// Module: crate::hashmap::algorithms
// Provides: {"compute_index"}
// Dependencies: {}
# [doc = " Calculate the index using (f0, f1), (d0, d1) in modulo m."] # [doc = " Returns [`None`] if d is (0, 0) or modulo is 0"] # [doc = " else returns the index computed using (f0 + f1 * d0 + d1) mod m."] pub fn compute_index (f : (u32 , u32) , d : (u32 , u32) , m : u32) -> Option < usize > { if d == (0 , 0) || m == 0 { None } else { Some ((f . 1 . wrapping_mul (d . 0) . wrapping_add (f . 0) . wrapping_add (d . 1) % m) as usize) } }
};
}
