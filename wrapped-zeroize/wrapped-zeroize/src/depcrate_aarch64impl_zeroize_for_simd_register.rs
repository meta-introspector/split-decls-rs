// Generated macro for impl_zeroize_for_simd_register (macro)
macro_rules! Depcrate_aarch64impl_zeroize_for_simd_register {
() => {
// Module: crate::aarch64
// Provides: {"impl_zeroize_for_simd_register"}
// Dependencies: {}
macro_rules ! impl_zeroize_for_simd_register { ($ ($ type : ty) ,* $ (,) ?) => { $ (impl Zeroize for $ type { # [inline] fn zeroize (& mut self) { volatile_write (self , unsafe { core :: mem :: zeroed () }) ; atomic_fence () ; } }) + } ; }
};
}
