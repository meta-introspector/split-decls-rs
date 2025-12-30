// Generated macro for impl_519 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_519 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_519"}
// Dependencies: {}
impl < T > fmt :: Debug for ZeroSlice < T > where T : AsULE + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . as_zerovec () . fmt (f) } }
};
}
