// Generated macro for impl_21 (impl)
macro_rules! Depcrate_vsockimpl_21 {
() => {
// Module: crate::vsock
// Provides: {"impl_21"}
// Dependencies: {}
impl Drop for VsockListener { fn drop (& mut self) { unsafe { let _ = close (self . fd . as_raw_fd ()) ; } } }
};
}
