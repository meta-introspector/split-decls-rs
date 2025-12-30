// Generated macro for impl_26 (impl)
macro_rules! Depcrate_vsockimpl_26 {
() => {
// Module: crate::vsock
// Provides: {"impl_26"}
// Dependencies: {}
impl Drop for VsockStream { fn drop (& mut self) { unsafe { let _ = close (self . fd . as_raw_fd ()) ; } } }
};
}
