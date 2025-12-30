// Generated macro for PollRecvData (struct)
macro_rules! Depcrate_quic_routerPollRecvData {
() => {
// Module: crate::quic::router
// Provides: {"PollRecvData"}
// Dependencies: {}
# [derive (Debug)] struct PollRecvData { bytes : usize , src_addr : SocketAddr , dst_addr_override : Option < SocketAddr > , rx_time : Option < SystemTime > , gro : Option < i32 > , # [cfg (target_os = "linux")] so_mark_data : Option < [u8 ; 4] > , }
};
}
