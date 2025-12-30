// Generated macro for impl_971 (impl)
macro_rules! Depcrate_net_socket_addr_anyimpl_971 {
() => {
// Module: crate::net::socket_addr_any
// Provides: {"impl_971"}
// Dependencies: {}
impl TryFrom < SocketAddrAny > for SocketAddrV6 { type Error = Errno ; # [doc = " Convert if the address is an IPv6 address."] # [doc = ""] # [doc = " Returns `Err(Errno::AFNOSUPPORT)` if the address family is not IPv6."] # [inline] fn try_from (value : SocketAddrAny) -> Result < Self , Self :: Error > { read_sockaddr :: read_sockaddr_v6 (& value) } }
};
}
