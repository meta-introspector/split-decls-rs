// Generated macro for impl_164 (impl)
macro_rules! Depcrate_tinyvecimpl_164 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_164"}
// Dependencies: {}
impl < 'p , A , I > ExactSizeIterator for TinyVecSplice < 'p , A , I > where A : Array , I : Iterator < Item = A :: Item > , { # [inline] fn len (& self) -> usize { self . removal_end - self . removal_start } }
};
}
