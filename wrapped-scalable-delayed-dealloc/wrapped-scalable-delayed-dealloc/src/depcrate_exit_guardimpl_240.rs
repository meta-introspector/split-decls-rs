// Generated macro for impl_240 (impl)
macro_rules! Depcrate_exit_guardimpl_240 {
() => {
// Module: crate::exit_guard
// Provides: {"impl_240"}
// Dependencies: {}
impl < T , F : FnOnce (T) > Drop for ExitGuard < T , F > { # [inline] fn drop (& mut self) { let (c , f) = unsafe { ManuallyDrop :: take (& mut self . drop_callback) } ; f (c) ; } }
};
}
