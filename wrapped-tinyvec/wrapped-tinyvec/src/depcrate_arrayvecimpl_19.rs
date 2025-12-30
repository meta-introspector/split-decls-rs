// Generated macro for impl_19 (impl)
macro_rules! Depcrate_arrayvecimpl_19 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_19"}
// Dependencies: {}
impl < A : Array > Deref for ArrayVec < A > { type Target = [A :: Item] ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . data . as_slice () [.. self . len as usize] } }
};
}
