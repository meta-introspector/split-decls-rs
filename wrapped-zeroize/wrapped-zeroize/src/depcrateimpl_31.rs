// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
# [doc = " Impl [`Zeroize`] on arrays of types that impl [`Zeroize`]."] impl < Z , const N : usize > Zeroize for [Z ; N] where Z : Zeroize , { fn zeroize (& mut self) { self . iter_mut () . zeroize () ; } }
};
}
