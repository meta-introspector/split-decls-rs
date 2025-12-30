// Generated macro for impl_39 (impl)
macro_rules! Depcrate_derimpl_39 {
() => {
// Module: crate::der
// Provides: {"impl_39"}
// Dependencies: {}
impl BitStringFlags < '_ > { pub (crate) fn bit_set (& self , bit : usize) -> bool { let byte_index = bit / 8 ; let bit_shift = 7 - (bit % 8) ; if self . raw_bits . len () < (byte_index + 1) { false } else { ((self . raw_bits [byte_index] >> bit_shift) & 1) != 0 } } }
};
}
