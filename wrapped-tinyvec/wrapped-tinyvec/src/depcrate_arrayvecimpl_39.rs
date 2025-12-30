// Generated macro for impl_39 (impl)
macro_rules! Depcrate_arrayvecimpl_39 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_39"}
// Dependencies: {}
impl < A : Array > AsMut < [A :: Item] > for ArrayVec < A > { # [inline (always)] fn as_mut (& mut self) -> & mut [A :: Item] { & mut * self } }
};
}
