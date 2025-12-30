// Generated macro for impl_53 (impl)
macro_rules! Depcrate_socketimpl_53 {
() => {
// Module: crate::socket
// Provides: {"impl_53"}
// Dependencies: {}
impl Domain { # [doc = " Domain for IPv4 communication, corresponding to `AF_INET`."] pub fn ipv4 () -> Domain { Domain (c :: AF_INET) } # [doc = " Domain for IPv6 communication, corresponding to `AF_INET6`."] pub fn ipv6 () -> Domain { Domain (c :: AF_INET6) } # [doc = " Domain for Unix socket communication, corresponding to `AF_UNIX`."] # [doc = ""] # [doc = " This function is only available on Unix when the `unix` feature is"] # [doc = " activated."] # [cfg (all (unix , feature = "unix"))] pub fn unix () -> Domain { Domain (c :: AF_UNIX) } }
};
}
