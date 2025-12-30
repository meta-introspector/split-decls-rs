// Generated macro for impl_108 (impl)
macro_rules! Depcrate_listenerimpl_108 {
() => {
// Module: crate::listener
// Provides: {"impl_108"}
// Dependencies: {}
impl Drop for Listener { fn drop (& mut self) { let _sess = self . sess . lock () ; unsafe { let _ = raw :: libssh2_channel_forward_cancel (self . raw) ; } } }
};
}
