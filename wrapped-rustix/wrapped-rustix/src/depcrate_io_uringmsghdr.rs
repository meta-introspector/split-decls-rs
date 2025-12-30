// Generated macro for MsgHdr (struct)
macro_rules! Depcrate_io_uringMsgHdr {
() => {
// Module: crate::io_uring
// Provides: {"MsgHdr"}
// Dependencies: {}
# [doc = " `msghdr`"] # [allow (missing_docs)] # [repr (C)] pub struct MsgHdr { pub msg_name : * mut c_void , pub msg_namelen : SocketAddrLen , pub msg_iov : * mut iovec , pub msg_iovlen : usize , pub msg_control : * mut c_void , pub msg_controllen : usize , pub msg_flags : RecvFlags , }
};
}
