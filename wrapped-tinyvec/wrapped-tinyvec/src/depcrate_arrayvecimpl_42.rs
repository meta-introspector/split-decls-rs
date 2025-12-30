// Generated macro for impl_42 (impl)
macro_rules! Depcrate_arrayvecimpl_42 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_42"}
// Dependencies: {}
impl < A : Array > BorrowMut < [A :: Item] > for ArrayVec < A > { # [inline (always)] fn borrow_mut (& mut self) -> & mut [A :: Item] { & mut * self } }
};
}
