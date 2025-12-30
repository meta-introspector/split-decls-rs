// Generated macro for impl_611 (impl)
macro_rules! Depcrate_streamimpl_611 {
() => {
// Module: crate::stream
// Provides: {"impl_611"}
// Dependencies: {}
impl < T : Clone , S > Clone for Checkpoint < T , S > { # [inline (always)] fn clone (& self) -> Self { Self { inner : self . inner . clone () , stream : Default :: default () , } } }
};
}
