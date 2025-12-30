// Generated macro for io_uring_recvmsg_out (struct)
macro_rules! Depcrate_io_uringio_uring_recvmsg_out {
() => {
// Module: crate::io_uring
// Provides: {"io_uring_recvmsg_out"}
// Dependencies: {}
# [allow (missing_docs)] # [repr (C)] # [derive (Debug , Default , Copy , Clone)] pub struct io_uring_recvmsg_out { pub namelen : SocketAddrLen , pub controllen : u32 , pub payloadlen : u32 , pub flags : RecvmsgOutFlags , }
};
}
