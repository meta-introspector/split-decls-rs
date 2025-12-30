// Generated macro for impl_43 (impl)
macro_rules! Depcrate_arrayvecimpl_43 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_43"}
// Dependencies: {}
impl < A : Array > Extend < A :: Item > for ArrayVec < A > { # [inline] fn extend < T : IntoIterator < Item = A :: Item > > (& mut self , iter : T) { for t in iter { self . push (t) } } }
};
}
