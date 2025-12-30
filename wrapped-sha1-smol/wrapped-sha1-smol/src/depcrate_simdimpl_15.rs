// Generated macro for impl_15 (impl)
macro_rules! Depcrate_simdimpl_15 {
() => {
// Module: crate::simd
// Provides: {"impl_15"}
// Dependencies: {}
impl Shr < usize > for u32x4 { type Output = u32x4 ; fn shr (self , amt : usize) -> u32x4 { u32x4 (self . 0 >> amt , self . 1 >> amt , self . 2 >> amt , self . 3 >> amt) } }
};
}
