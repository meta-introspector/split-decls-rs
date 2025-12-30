// Generated macro for impl_875 (impl)
macro_rules! Depcrate_socket_connectedimpl_875 {
() => {
// Module: crate::socket::connected
// Provides: {"impl_875"}
// Dependencies: {}
impl TryFrom < UdpSocket > for Socket < Arc < UdpSocket > , Arc < UdpSocket > > { type Error = io :: Error ; fn try_from (socket : UdpSocket) -> Result < Self , Self :: Error > { Self :: from_udp (socket) } }
};
}
