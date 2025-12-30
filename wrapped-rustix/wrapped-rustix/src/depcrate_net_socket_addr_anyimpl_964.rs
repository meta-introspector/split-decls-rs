// Generated macro for impl_964 (impl)
macro_rules! Depcrate_net_socket_addr_anyimpl_964 {
() => {
// Module: crate::net::socket_addr_any
// Provides: {"impl_964"}
// Dependencies: {}
impl fmt :: Debug for SocketAddrAny { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . address_family () { AddressFamily :: INET => { if let Ok (addr) = SocketAddrV4 :: try_from (self . clone ()) { return addr . fmt (f) ; } } AddressFamily :: INET6 => { if let Ok (addr) = SocketAddrV6 :: try_from (self . clone ()) { return addr . fmt (f) ; } } # [cfg (unix)] AddressFamily :: UNIX => { if let Ok (addr) = SocketAddrUnix :: try_from (self . clone ()) { return addr . fmt (f) ; } } # [cfg (target_os = "linux")] AddressFamily :: XDP => { if let Ok (addr) = crate :: net :: xdp :: SocketAddrXdp :: try_from (self . clone ()) { return addr . fmt (f) ; } } # [cfg (linux_kernel)] AddressFamily :: NETLINK => { if let Ok (addr) = crate :: net :: netlink :: SocketAddrNetlink :: try_from (self . clone ()) { return addr . fmt (f) ; } } _ => { } } f . debug_struct ("SocketAddrAny") . field ("address_family" , & self . address_family ()) . field ("namelen" , & self . addr_len ()) . finish () } }
};
}
