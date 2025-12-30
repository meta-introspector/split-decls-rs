// Generated macro for impl_121 (impl)
macro_rules! Depcrate_low_level_pipeimpl_121 {
() => {
// Module: crate::low_level::pipe
// Provides: {"impl_121"}
// Dependencies: {}
impl Drop for WakeFd { fn drop (& mut self) { unsafe { libc :: close (self . fd) ; } } }
};
}
