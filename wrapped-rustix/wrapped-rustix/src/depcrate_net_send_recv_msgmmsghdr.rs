// Generated macro for MMsgHdr (struct)
macro_rules! Depcrate_net_send_recv_msgMMsgHdr {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"MMsgHdr"}
// Dependencies: {}
# [doc = " An ABI-compatible wrapper for `mmsghdr`, for sending multiple messages with"] # [doc = " [sendmmsg]."] # [cfg (target_os = "linux")] # [repr (transparent)] pub struct MMsgHdr < 'a > { raw : c :: mmsghdr , _phantom : PhantomData < & 'a mut () > , }
};
}
