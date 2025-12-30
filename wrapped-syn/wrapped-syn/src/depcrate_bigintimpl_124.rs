// Generated macro for impl_124 (impl)
macro_rules! Depcrate_bigintimpl_124 {
() => {
// Module: crate::bigint
// Provides: {"impl_124"}
// Dependencies: {}
impl AddAssign < u8 > for BigInt { fn add_assign (& mut self , mut increment : u8) { self . reserve_two_digits () ; let mut i = 0 ; while increment > 0 { let sum = self . digits [i] + increment ; self . digits [i] = sum % 10 ; increment = sum / 10 ; i += 1 ; } } }
};
}
