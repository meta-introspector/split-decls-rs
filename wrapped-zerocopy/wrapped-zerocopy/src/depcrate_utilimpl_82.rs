// Generated macro for impl_82 (impl)
macro_rules! Depcrate_utilimpl_82 {
() => {
// Module: crate::util
// Provides: {"impl_82"}
// Dependencies: {}
impl < T : ? Sized > AsAddress for & T { # [inline (always)] fn addr (self) -> usize { let ptr : * const T = self ; AsAddress :: addr (ptr) } }
};
}
