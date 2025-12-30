// Generated macro for impl_174 (impl)
macro_rules! Depcrate_tinyvecimpl_174 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_174"}
// Dependencies: {}
impl < A : Array > From < A > for TinyVec < A > { # [inline] fn from (array : A) -> Self { TinyVec :: Inline (ArrayVec :: from (array)) } }
};
}
