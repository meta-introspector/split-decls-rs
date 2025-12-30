// Generated macro for impl_932 (impl)
macro_rules! Depcrate_unix_network_helperimpl_932 {
() => {
// Module: crate::unix::network_helper
// Provides: {"impl_932"}
// Dependencies: {}
impl Drop for InterfaceAddressIterator { fn drop (& mut self) { unsafe { libc :: freeifaddrs (self . buf) ; } } }
};
}
