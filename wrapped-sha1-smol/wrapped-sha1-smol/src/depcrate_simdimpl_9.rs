// Generated macro for impl_9 (impl)
macro_rules! Depcrate_simdimpl_9 {
() => {
// Module: crate::simd
// Provides: {"impl_9"}
// Dependencies: {}
impl Sub for u32x4 { type Output = u32x4 ; fn sub (self , rhs : u32x4) -> u32x4 { u32x4 (self . 0 . wrapping_sub (rhs . 0) , self . 1 . wrapping_sub (rhs . 1) , self . 2 . wrapping_sub (rhs . 2) , self . 3 . wrapping_sub (rhs . 3) ,) } }
};
}
