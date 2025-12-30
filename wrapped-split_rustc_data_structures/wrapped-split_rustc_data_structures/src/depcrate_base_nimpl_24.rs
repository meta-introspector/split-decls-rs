// Generated macro for impl_24 (impl)
macro_rules! Depcrate_base_nimpl_24 {
() => {
// Module: crate::base_n
// Provides: {"impl_24"}
// Dependencies: {}
impl ToBaseN for u128 { fn encoded_len (base : usize) -> usize { let mut max = u128 :: MAX ; let mut len = 0 ; while max > 0 { len += 1 ; max /= base as u128 ; } len } }
};
}
