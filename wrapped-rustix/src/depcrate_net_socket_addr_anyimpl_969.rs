// Generated macro for impl_969 (impl)
macro_rules! Depcrate_net_socket_addr_anyimpl_969 {
() => {
// Module: crate::net::socket_addr_any
// Provides: {"impl_969"}
// Dependencies: {}
impl TryFrom < SocketAddrAny > for SocketAddrV4 { type Error = Errno ; # [doc = " Convert if the address is an IPv4 address."] # [doc = ""] # [doc = " Returns `Err(Errno::AFNOSUPPORT)` if the address family is not IPv4."] # [inline] fn try_from (value : SocketAddrAny) -> Result < Self , Self :: Error > { read_sockaddr :: read_sockaddr_v4 (& value) } }
};
}
