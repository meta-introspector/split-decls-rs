// Generated macro for send_to (function)
macro_rules! Depcrate_quic_io_gsosend_to {
() => {
// Module: crate::quic::io::gso
// Provides: {"send_to"}
// Dependencies: {}
# [cfg (any (not (target_os = "linux") , feature = "fuzzing"))] # [allow (clippy :: too_many_arguments)] pub (crate) async fn send_to (socket : & tokio :: net :: UdpSocket , to : SocketAddr , _from : Option < SocketAddr > , send_buf : & [u8] , _segment_size : usize , _tx_time : Option < Instant > , _would_block_metric : Counter , _send_to_wouldblock_duration_s : TimeHistogram ,) -> io :: Result < usize > { socket . send_to (send_buf , to) . await }
};
}
