// Generated macro for impl_1566 (impl)
macro_rules! Depcrate_os_unix_net_listenerimpl_1566 {
() => {
// Module: crate::os::unix::net::listener
// Provides: {"impl_1566"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl < 'a > Iterator for Incoming < 'a > { type Item = io :: Result < UnixStream > ; fn next (& mut self) -> Option < io :: Result < UnixStream > > { Some (self . listener . accept () . map (| s | s . 0)) } fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
};
}
