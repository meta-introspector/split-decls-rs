// Generated macro for impl_168 (impl)
macro_rules! Depcrate_tinyvecimpl_168 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_168"}
// Dependencies: {}
impl < A : Array > AsMut < [A :: Item] > for TinyVec < A > { # [inline (always)] fn as_mut (& mut self) -> & mut [A :: Item] { & mut * self } }
};
}
