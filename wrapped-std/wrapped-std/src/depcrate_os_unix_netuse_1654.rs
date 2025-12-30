// Generated macro for use_1654 (pub_use)
macro_rules! Depcrate_os_unix_netuse_1654 {
() => {
// Module: crate::os::unix::net
// Provides: {"use_1654"}
// Dependencies: {}
# [cfg (any (target_os = "android" , target_os = "linux" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "netbsd" , target_os = "openbsd" , target_os = "nto" , target_vendor = "apple" , target_os = "cygwin" ,))] # [unstable (feature = "peer_credentials_unix_socket" , issue = "42839" , reason = "unstable")] pub use self :: ucred :: * ;
};
}
