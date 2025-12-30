// Generated macro for impl_8 (impl)
macro_rules! Depcrate_simdimpl_8 {
() => {
// Module: crate::simd
// Provides: {"impl_8"}
// Dependencies: {}
impl Add for u32x4 { type Output = u32x4 ; fn add (self , rhs : u32x4) -> u32x4 { u32x4 (self . 0 . wrapping_add (rhs . 0) , self . 1 . wrapping_add (rhs . 1) , self . 2 . wrapping_add (rhs . 2) , self . 3 . wrapping_add (rhs . 3) ,) } }
};
}
