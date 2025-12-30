// Generated macro for impl_zeroize_for_non_zero (macro)
macro_rules! Depcrateimpl_zeroize_for_non_zero {
() => {
// Module: crate
// Provides: {"impl_zeroize_for_non_zero"}
// Dependencies: {}
macro_rules ! impl_zeroize_for_non_zero { ($ ($ type : ty) ,+) => { $ (impl Zeroize for $ type { fn zeroize (& mut self) { const ONE : $ type = match <$ type >:: new (1) { Some (one) => one , None => unreachable ! () , } ; volatile_write (self , ONE) ; atomic_fence () ; } }) + } ; }
};
}
