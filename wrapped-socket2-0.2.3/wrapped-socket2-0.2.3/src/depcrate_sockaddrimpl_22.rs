// Generated macro for impl_22 (impl)
macro_rules! Depcrate_sockaddrimpl_22 {
() => {
// Module: crate::sockaddr
// Provides: {"impl_22"}
// Dependencies: {}
impl From < SocketAddrV6 > for SockAddr { fn from (addr : SocketAddrV6) -> SockAddr { unsafe { SockAddr :: from_raw_parts (& addr as * const _ as * const _ , mem :: size_of :: < SocketAddrV6 > () as socklen_t) } } }
};
}
