// Generated macro for impl_887 (impl)
macro_rules! Depcrate_socket_listenerimpl_887 {
() => {
// Module: crate::socket::listener
// Provides: {"impl_887"}
// Dependencies: {}
impl TryFrom < std :: net :: UdpSocket > for QuicListener { type Error = io :: Error ; fn try_from (socket : std :: net :: UdpSocket) -> Result < Self , Self :: Error > { socket . set_nonblocking (true) ? ; let socket = UdpSocket :: from_std (socket) ? ; Self :: try_from (socket) } }
};
}
