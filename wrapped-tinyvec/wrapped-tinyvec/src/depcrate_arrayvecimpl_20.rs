// Generated macro for impl_20 (impl)
macro_rules! Depcrate_arrayvecimpl_20 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_20"}
// Dependencies: {}
impl < A : Array > DerefMut for ArrayVec < A > { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . data . as_slice_mut () [.. self . len as usize] } }
};
}
