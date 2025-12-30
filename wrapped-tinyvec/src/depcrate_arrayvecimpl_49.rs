// Generated macro for impl_49 (impl)
macro_rules! Depcrate_arrayvecimpl_49 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_49"}
// Dependencies: {}
impl < A : Array > FromIterator < A :: Item > for ArrayVec < A > { # [inline] fn from_iter < T : IntoIterator < Item = A :: Item > > (iter : T) -> Self { let mut av = Self :: default () ; for i in iter { av . push (i) } av } }
};
}
