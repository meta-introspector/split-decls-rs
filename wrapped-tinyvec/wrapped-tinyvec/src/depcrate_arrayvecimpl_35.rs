// Generated macro for impl_35 (impl)
macro_rules! Depcrate_arrayvecimpl_35 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'p , A , I > ExactSizeIterator for ArrayVecSplice < 'p , A , I > where A : Array , I : Iterator < Item = A :: Item > , { # [inline] fn len (& self) -> usize { self . removal_end - self . removal_start } }
};
}
