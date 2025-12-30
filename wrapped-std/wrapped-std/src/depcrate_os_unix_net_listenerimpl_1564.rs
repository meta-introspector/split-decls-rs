// Generated macro for impl_1564 (impl)
macro_rules! Depcrate_os_unix_net_listenerimpl_1564 {
() => {
// Module: crate::os::unix::net::listener
// Provides: {"impl_1564"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl < 'a > IntoIterator for & 'a UnixListener { type Item = io :: Result < UnixStream > ; type IntoIter = Incoming < 'a > ; fn into_iter (self) -> Incoming < 'a > { self . incoming () } }
};
}
