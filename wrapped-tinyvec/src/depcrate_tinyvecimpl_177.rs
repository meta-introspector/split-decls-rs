// Generated macro for impl_177 (impl)
macro_rules! Depcrate_tinyvecimpl_177 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_177"}
// Dependencies: {}
impl < A : Array > FromIterator < A :: Item > for TinyVec < A > { # [inline] fn from_iter < T : IntoIterator < Item = A :: Item > > (iter : T) -> Self { let mut av = Self :: default () ; av . extend (iter) ; av } }
};
}
