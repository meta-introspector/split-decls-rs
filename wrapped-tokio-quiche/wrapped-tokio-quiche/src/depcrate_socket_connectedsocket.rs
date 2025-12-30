// Generated macro for Socket (struct)
macro_rules! Depcrate_socket_connectedSocket {
() => {
// Module: crate::socket::connected
// Provides: {"Socket"}
// Dependencies: {}
# [doc = " A connected datagram socket with separate `send` and `recv` halves."] # [doc = ""] # [doc = " [`Socket`] abstracts over both real UDP-based connections and in-process"] # [doc = " tunneled flows like (multi-hop) MASQUE flows. It uses the"] # [doc = " [`datagram_socket`] traits for this purpose."] # [derive (Debug)] pub struct Socket < Tx , Rx > { # [doc = " The sending half of the connection. This generally supports concurrent"] # [doc = " senders."] pub send : Tx , # [doc = " The receiving half of the connection. This is generally owned by a"] # [doc = " single caller."] pub recv : Rx , # [doc = " The address of the local endpoint."] pub local_addr : SocketAddr , # [doc = " The address of the remote endpoint."] pub peer_addr : SocketAddr , # [doc = " The [`SocketCapabilities`] to use for this socket."] # [doc = ""] # [doc = " By default, [`Socket`]s are constructed with all capabilities"] # [doc = " disabled. On Linux, you can use `apply_max_capabilities()` to (try"] # [doc = " to) enable all supported capabilities."] pub capabilities : SocketCapabilities , }
};
}
