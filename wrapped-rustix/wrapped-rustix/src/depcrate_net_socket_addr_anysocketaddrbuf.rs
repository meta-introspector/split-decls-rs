// Generated macro for SocketAddrBuf (struct)
macro_rules! Depcrate_net_socket_addr_anySocketAddrBuf {
() => {
// Module: crate::net::socket_addr_any
// Provides: {"SocketAddrBuf"}
// Dependencies: {}
# [doc = " Temporary buffer for creating a `SocketAddrAny` from a syscall that writes"] # [doc = " to a `sockaddr_t` and `socklen_t`"] # [doc = ""] # [doc = " Unlike `SocketAddrAny`, this does not maintain the invariant that `len`"] # [doc = " bytes are initialized."] pub (crate) struct SocketAddrBuf { pub (crate) len : c :: socklen_t , pub (crate) storage : MaybeUninit < SocketAddrStorage > , }
};
}
