// Generated macro for impl_873 (impl)
macro_rules! Depcrate_socket_connectedimpl_873 {
() => {
// Module: crate::socket::connected
// Provides: {"impl_873"}
// Dependencies: {}
impl < Tx , Rx > Socket < Tx , Rx > { # [doc = " Creates a [`Socket`] from a [`UdpSocket`] by wrapping the file"] # [doc = " descriptor in an [`Arc`]."] pub fn from_udp (socket : UdpSocket ,) -> io :: Result < Socket < Arc < UdpSocket > , Arc < UdpSocket > > > { let local_addr = socket . local_addr () ? ; let peer_addr = socket . peer_addr () ? ; let send = Arc :: new (socket) ; let recv = Arc :: clone (& send) ; Ok (Socket { send , recv , local_addr , peer_addr , capabilities : SocketCapabilities :: default () , }) } }
};
}
