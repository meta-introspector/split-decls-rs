// Generated macro for impl_21 (impl)
macro_rules! Depcrate_sockaddrimpl_21 {
() => {
// Module: crate::sockaddr
// Provides: {"impl_21"}
// Dependencies: {}
impl From < SocketAddrV4 > for SockAddr { fn from (addr : SocketAddrV4) -> SockAddr { unsafe { SockAddr :: from_raw_parts (& addr as * const _ as * const _ , mem :: size_of :: < SocketAddrV4 > () as socklen_t) } } }
};
}
