// Generated macro for impl_1513 (impl)
macro_rules! Depcrate_os_unix_net_ancillaryimpl_1513 {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"impl_1513"}
// Dependencies: {}
# [cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "netbsd" , target_os = "freebsd" , target_os = "cygwin" ,))] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] impl < 'a > Iterator for ScmCredentials < 'a > { type Item = SocketCred ; fn next (& mut self) -> Option < SocketCred > { Some (SocketCred (self . 0 . next () ?)) } }
};
}
