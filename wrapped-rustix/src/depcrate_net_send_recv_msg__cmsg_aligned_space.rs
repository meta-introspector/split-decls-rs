// Generated macro for __cmsg_aligned_space (function)
macro_rules! Depcrate_net_send_recv_msg__cmsg_aligned_space {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"__cmsg_aligned_space"}
// Dependencies: {}
# [doc = " Helper function for [`cmsg_aligned_space`]."] # [doc (hidden)] pub const fn __cmsg_aligned_space (len : usize) -> usize { let converted_len = len as u32 ; if converted_len as usize != len { unreachable ! () ; } unsafe { c :: CMSG_SPACE (converted_len) as usize } }
};
}
