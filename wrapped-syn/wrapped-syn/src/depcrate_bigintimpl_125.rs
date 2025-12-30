// Generated macro for impl_125 (impl)
macro_rules! Depcrate_bigintimpl_125 {
() => {
// Module: crate::bigint
// Provides: {"impl_125"}
// Dependencies: {}
impl MulAssign < u8 > for BigInt { fn mul_assign (& mut self , base : u8) { self . reserve_two_digits () ; let mut carry = 0 ; for digit in & mut self . digits { let prod = * digit * base + carry ; * digit = prod % 10 ; carry = prod / 10 ; } } }
};
}
