// Generated macro for impl_107 (impl)
macro_rules! Depcrateimpl_107 {
() => {
// Module: crate
// Provides: {"impl_107"}
// Dependencies: {}
# [cfg (not (target_os = "redox"))] impl RecvFlags { # [doc = " Check if the message contains a truncated datagram."] # [doc = ""] # [doc = " This flag is only used for datagram-based sockets,"] # [doc = " not for stream sockets."] # [doc = ""] # [doc = " On Unix this corresponds to the `MSG_TRUNC` flag."] # [doc = " On Windows this corresponds to the `WSAEMSGSIZE` error code."] # [cfg (not (target_os = "espidf"))] pub const fn is_truncated (self) -> bool { self . 0 & sys :: MSG_TRUNC != 0 } }
};
}
