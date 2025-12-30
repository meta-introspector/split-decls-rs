// Generated macro for div_ceil (function)
macro_rules! Depcrate_libs_bitvecdiv_ceil {
() => {
// Module: crate::libs::bitvec
// Provides: {"div_ceil"}
// Dependencies: {}
# [inline] pub const fn div_ceil (n : usize , rhs : usize) -> usize { let d = n / rhs ; let r = n % rhs ; if r > 0 { d + 1 } else { d } }
};
}
