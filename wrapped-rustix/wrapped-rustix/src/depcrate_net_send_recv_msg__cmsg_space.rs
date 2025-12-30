// Generated macro for __cmsg_space (function)
macro_rules! Depcrate_net_send_recv_msg__cmsg_space {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"__cmsg_space"}
// Dependencies: {}
# [doc = " Helper function for [`cmsg_space`]."] # [doc (hidden)] pub const fn __cmsg_space (len : usize) -> usize { let len = len + align_of :: < c :: cmsghdr > () ; __cmsg_aligned_space (len) }
};
}
