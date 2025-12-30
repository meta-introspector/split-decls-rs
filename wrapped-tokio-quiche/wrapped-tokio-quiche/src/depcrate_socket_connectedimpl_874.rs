// Generated macro for impl_874 (impl)
macro_rules! Depcrate_socket_connectedimpl_874 {
() => {
// Module: crate::socket::connected
// Provides: {"impl_874"}
// Dependencies: {}
impl < Tx , Rx > Socket < Tx , Rx > where Tx : DatagramSocketSend , Rx : DatagramSocketRecv , { # [doc = " Checks whether both `send` and `recv` refer to the same underlying"] # [doc = " UDP socket FD and returns a reference to that socket."] # [doc = ""] # [doc = " # Note"] # [doc = " The file descriptor _numbers_ have to be identical. A pair of FDs"] # [doc = " created by [`dup(2)`](https://man7.org/linux/man-pages/man2/dup.2.html) will"] # [doc = " return `None`."] # [cfg (unix)] pub fn as_udp_socket (& self) -> Option < & UdpSocket > { use std :: os :: fd :: AsRawFd ; let send = self . send . as_udp_socket () ? ; let recv = self . recv . as_udp_socket () ? ; (send . as_raw_fd () == recv . as_raw_fd ()) . then_some (send) } # [doc = " Tries to enable all sockopts supported by the crate for this socket."] # [doc = ""] # [doc = " This does nothing unless `send` and `recv` refer to the same UDP socket"] # [doc = " FD. See `SocketCapabilities::apply_all_and_get_compatibility` for"] # [doc = " details."] # [cfg (target_os = "linux")] pub fn apply_max_capabilities (& mut self) { let Some (socket) = self . as_udp_socket () else { return ; } ; let capabilities = SocketCapabilities :: apply_all_and_get_compatibility (socket) ; self . capabilities = capabilities ; } }
};
}
