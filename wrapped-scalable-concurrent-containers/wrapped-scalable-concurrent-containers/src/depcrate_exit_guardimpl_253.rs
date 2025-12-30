// Generated macro for impl_253 (impl)
macro_rules! Depcrate_exit_guardimpl_253 {
() => {
// Module: crate::exit_guard
// Provides: {"impl_253"}
// Dependencies: {}
impl < T , F : FnOnce (T) > DerefMut for ExitGuard < T , F > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . drop_callback . 0 } }
};
}
