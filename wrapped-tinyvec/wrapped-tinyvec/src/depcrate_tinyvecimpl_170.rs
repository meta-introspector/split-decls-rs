// Generated macro for impl_170 (impl)
macro_rules! Depcrate_tinyvecimpl_170 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_170"}
// Dependencies: {}
impl < A : Array > Borrow < [A :: Item] > for TinyVec < A > { # [inline (always)] fn borrow (& self) -> & [A :: Item] { & * self } }
};
}
