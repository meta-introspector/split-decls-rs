// Generated macro for impl_41 (impl)
macro_rules! Depcrate_arrayvecimpl_41 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_41"}
// Dependencies: {}
impl < A : Array > Borrow < [A :: Item] > for ArrayVec < A > { # [inline (always)] fn borrow (& self) -> & [A :: Item] { & * self } }
};
}
