// Generated macro for impl_16 (impl)
macro_rules! Depcrate_simdimpl_16 {
() => {
// Module: crate::simd
// Provides: {"impl_16"}
// Dependencies: {}
impl Shr < u32x4 > for u32x4 { type Output = u32x4 ; fn shr (self , rhs : u32x4) -> u32x4 { u32x4 (self . 0 >> rhs . 0 , self . 1 >> rhs . 1 , self . 2 >> rhs . 2 , self . 3 >> rhs . 3 ,) } }
};
}
