// Generated macro for impl_352 (impl)
macro_rules! Depcrate_thread_globalimpl_352 {
() => {
// Module: crate::thread::global
// Provides: {"impl_352"}
// Dependencies: {}
impl Global { # [doc = " Executes the given `task` with [`Global`]."] pub (super) fn with < R > (task : impl FnOnce (& Self) -> R) -> R { GLOBAL . with (task) } # [doc = " Converts the global type to [`WindowOrWorkerExt`] when appropriate and"] # [doc = " executes the given `task` with it."] pub (super) fn with_window_or_worker < R > (task : impl FnOnce (& WindowOrWorkerExt) -> R ,) -> Option < R > { GLOBAL . with (| global | { let global : & WindowOrWorkerExt = match global { Self :: Window (window) => window . unchecked_ref () , Self :: Dedicated (worker) => worker . unchecked_ref () , Self :: Service (worker) | Self :: Worker (worker) => worker . unchecked_ref () , Self :: Shared (worker) => worker . unchecked_ref () , Self :: Worklet | Self :: Unknown => return None , } ; Some (task (global)) }) } }
};
}
