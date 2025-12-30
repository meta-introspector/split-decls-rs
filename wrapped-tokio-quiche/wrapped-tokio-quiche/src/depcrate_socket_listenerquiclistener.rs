// Generated macro for QuicListener (struct)
macro_rules! Depcrate_socket_listenerQuicListener {
() => {
// Module: crate::socket::listener
// Provides: {"QuicListener"}
// Dependencies: {}
# [doc = " Wrapper around a [`UdpSocket`] for server-side QUIC connections."] # [doc = ""] # [doc = " The wrapper carries socket-specific parameters, in contrast to the"] # [doc = " [`settings`](crate::settings) structs which apply to _all_ sockets"] # [doc = " for a given QUIC server."] # [doc = ""] # [doc = " To create a [`QuicListener`], you may either instantiate the struct yourself"] # [doc = " or use one of the `TryFrom` implementations."] # [derive (Debug)] pub struct QuicListener { # [doc = " The wrapped [tokio] socket."] pub socket : UdpSocket , # [doc = " An opaque value that is later passed to the"] # [doc = " [`ConnectionIdGenerator`](crate::ConnectionIdGenerator)."] pub socket_cookie : u64 , # [doc = " The [`SocketCapabilities`] to use for this socket."] # [doc = ""] # [doc = " By default, [`QuicListener`]s are constructed with all capabilities"] # [doc = " disabled. On Linux, you can use `apply_max_capabilities()` to (try"] # [doc = " to) enable all supported capabilities."] pub capabilities : SocketCapabilities , }
};
}
