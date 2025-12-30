// Generated macro for Incoming (struct)
macro_rules! Depcrate_quic_connectionIncoming {
() => {
// Module: crate::quic::connection
// Provides: {"Incoming"}
// Dependencies: {}
# [doc = " A received network packet with additional metadata."] # [derive (Debug)] pub struct Incoming { # [doc = " The address that sent the inbound packet."] pub peer_addr : SocketAddr , # [doc = " The address on which we received the inbound packet."] pub local_addr : SocketAddr , # [doc = " The receive timestamp of the packet."] # [doc = ""] # [doc = " Used for the `perf-quic-listener-metrics` feature."] pub rx_time : Option < SystemTime > , # [doc = " The packet's contents."] pub buf : PooledBuf , # [doc = " If set, then `buf` is a GRO buffer containing multiple packets."] # [doc = " Each individual packet has a size of `gso` (except for the last one)."] pub gro : Option < i32 > , # [doc = " [SO_MARK] control message value received from the socket."] # [doc = ""] # [doc = " This will always be `None` after the connection has been spawned as"] # [doc = " the message is `take()`d before spawning."] # [doc = ""] # [doc = " [SO_MARK]: https://man7.org/linux/man-pages/man7/socket.7.html"] # [cfg (target_os = "linux")] pub so_mark_data : Option < [u8 ; 4] > , }
};
}
