// Generated macro for impl_98 (impl)
macro_rules! Depcrate_knownhostsimpl_98 {
() => {
// Module: crate::knownhosts
// Provides: {"impl_98"}
// Dependencies: {}
impl Drop for KnownHosts { fn drop (& mut self) { let _sess = self . sess . lock () ; unsafe { raw :: libssh2_knownhost_free (self . raw) } } }
};
}
