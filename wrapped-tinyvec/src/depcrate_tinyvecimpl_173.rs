// Generated macro for impl_173 (impl)
macro_rules! Depcrate_tinyvecimpl_173 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_173"}
// Dependencies: {}
impl < A : Array > From < ArrayVec < A > > for TinyVec < A > { # [inline (always)] fn from (arr : ArrayVec < A >) -> Self { TinyVec :: Inline (arr) } }
};
}
