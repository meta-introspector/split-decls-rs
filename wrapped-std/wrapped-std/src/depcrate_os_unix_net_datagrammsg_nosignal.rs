// Generated macro for MSG_NOSIGNAL (const)
macro_rules! Depcrate_os_unix_net_datagramMSG_NOSIGNAL {
() => {
// Module: crate::os::unix::net::datagram
// Provides: {"MSG_NOSIGNAL"}
// Dependencies: {}
# [cfg (not (any (target_os = "linux" , target_os = "android" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "openbsd" , target_os = "netbsd" , target_os = "solaris" , target_os = "illumos" , target_os = "haiku" , target_os = "nto" , target_os = "cygwin")))] const MSG_NOSIGNAL : core :: ffi :: c_int = 0x0 ;
};
}
