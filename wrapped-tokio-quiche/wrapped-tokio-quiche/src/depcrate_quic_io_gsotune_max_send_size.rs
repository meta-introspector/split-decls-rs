// Generated macro for tune_max_send_size (function)
macro_rules! Depcrate_quic_io_gsotune_max_send_size {
() => {
// Module: crate::quic::io::gso
// Provides: {"tune_max_send_size"}
// Dependencies: {}
# [cfg (not (feature = "gcongestion"))] # [doc = " Returns a new max send buffer size to avoid the fragmentation"] # [doc = " at the end. Maximum send buffer size is min(MAX_SEND_BUF_SIZE,"] # [doc = " connection's send_quantum)."] # [doc = " For example,"] # [doc = ""] # [doc = " - max_send_buf = 1000 and mss = 100, return 1000"] # [doc = " - max_send_buf = 1000 and mss = 90, return 990"] # [doc = ""] # [doc = " not to have last 10 bytes packet."] pub (crate) fn tune_max_send_size (segment_size : Option < usize > , send_quantum : usize , max_capacity : usize ,) -> usize { let max_send_buf_size = send_quantum . min (max_capacity) ; if let Some (mss) = segment_size { max_send_buf_size / mss * mss } else { max_send_buf_size } }
};
}
