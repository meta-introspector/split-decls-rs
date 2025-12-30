// Generated macro for impl_171 (impl)
macro_rules! Depcrate_tinyvecimpl_171 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_171"}
// Dependencies: {}
impl < A : Array > BorrowMut < [A :: Item] > for TinyVec < A > { # [inline (always)] fn borrow_mut (& mut self) -> & mut [A :: Item] { & mut * self } }
};
}
