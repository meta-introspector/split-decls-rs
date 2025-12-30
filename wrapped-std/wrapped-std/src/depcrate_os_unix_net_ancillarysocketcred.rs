// Generated macro for SocketCred (struct)
macro_rules! Depcrate_os_unix_net_ancillarySocketCred {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"SocketCred"}
// Dependencies: {}
# [cfg (target_os = "freebsd")] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] # [derive (Clone)] pub struct SocketCred (libc :: sockcred2) ;
};
}
