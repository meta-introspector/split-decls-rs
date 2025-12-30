// Generated macro for impl_83 (impl)
macro_rules! Depcrate_utilimpl_83 {
() => {
// Module: crate::util
// Provides: {"impl_83"}
// Dependencies: {}
impl < T : ? Sized > AsAddress for & mut T { # [inline (always)] fn addr (self) -> usize { let ptr : * const T = self ; AsAddress :: addr (ptr) } }
};
}
