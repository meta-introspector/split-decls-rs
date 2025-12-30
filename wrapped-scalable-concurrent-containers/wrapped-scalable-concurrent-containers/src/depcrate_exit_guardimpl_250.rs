// Generated macro for impl_250 (impl)
macro_rules! Depcrate_exit_guardimpl_250 {
() => {
// Module: crate::exit_guard
// Provides: {"impl_250"}
// Dependencies: {}
impl < T , F : FnOnce (T) > ExitGuard < T , F > { # [inline] pub (crate) fn forget (mut self) { unsafe { ManuallyDrop :: drop (& mut self . drop_callback) ; } forget (self) ; } }
};
}
