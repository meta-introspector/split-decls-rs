// Generated macro for impl_886 (impl)
macro_rules! Depcrate_socket_listenerimpl_886 {
() => {
// Module: crate::socket::listener
// Provides: {"impl_886"}
// Dependencies: {}
impl TryFrom < UdpSocket > for QuicListener { type Error = io :: Error ; fn try_from (socket : UdpSocket) -> Result < Self , Self :: Error > { Ok (Self { socket , socket_cookie : 0 , capabilities : SocketCapabilities :: default () , }) } }
};
}
