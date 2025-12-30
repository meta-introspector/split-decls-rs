// Generated macro for impl_252 (impl)
macro_rules! Depcrate_exit_guardimpl_252 {
() => {
// Module: crate::exit_guard
// Provides: {"impl_252"}
// Dependencies: {}
impl < T , F : FnOnce (T) > Deref for ExitGuard < T , F > { type Target = T ; # [inline] fn deref (& self) -> & Self :: Target { & self . drop_callback . 0 } }
};
}
