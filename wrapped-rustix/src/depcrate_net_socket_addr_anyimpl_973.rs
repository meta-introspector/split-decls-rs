// Generated macro for impl_973 (impl)
macro_rules! Depcrate_net_socket_addr_anyimpl_973 {
() => {
// Module: crate::net::socket_addr_any
// Provides: {"impl_973"}
// Dependencies: {}
# [cfg (unix)] impl TryFrom < SocketAddrAny > for SocketAddrUnix { type Error = Errno ; # [doc = " Convert if the address is a Unix socket address."] # [doc = ""] # [doc = " Returns `Err(Errno::AFNOSUPPORT)` if the address family is not Unix."] # [inline] fn try_from (value : SocketAddrAny) -> Result < Self , Self :: Error > { read_sockaddr :: read_sockaddr_unix (& value) } }
};
}
