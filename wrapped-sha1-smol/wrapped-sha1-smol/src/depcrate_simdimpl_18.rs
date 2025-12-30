// Generated macro for impl_18 (impl)
macro_rules! Depcrate_simdimpl_18 {
() => {
// Module: crate::simd
// Provides: {"impl_18"}
// Dependencies: {}
impl Add for u64x2 { type Output = u64x2 ; fn add (self , rhs : u64x2) -> u64x2 { u64x2 (self . 0 . wrapping_add (rhs . 0) , self . 1 . wrapping_add (rhs . 1)) } }
};
}
