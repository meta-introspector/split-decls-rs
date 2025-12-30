// Generated macro for ExitGuard (struct)
macro_rules! Depcrate_exit_guardExitGuard {
() => {
// Module: crate::exit_guard
// Provides: {"ExitGuard"}
// Dependencies: {}
# [doc = " [`ExitGuard`] captures the environment and invokes a defined closure at the end of the scope."] pub (crate) struct ExitGuard < T , F : FnOnce (T) > { drop_callback : ManuallyDrop < (T , F) > , }
};
}
