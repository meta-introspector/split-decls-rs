// Generated macro for IoringMsgringCmds (enum)
macro_rules! Depcrate_io_uringIoringMsgringCmds {
() => {
// Module: crate::io_uring
// Provides: {"IoringMsgringCmds"}
// Dependencies: {}
# [doc = " `IORING_MSG_*` constants which represent commands for use with"] # [doc = " [`IoringOp::MsgRing`], (`seq.addr`)"] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (u64)] # [non_exhaustive] pub enum IoringMsgringCmds { # [doc = " `IORING_MSG_DATA`"] Data = sys :: io_uring_msg_ring_flags :: IORING_MSG_DATA as _ , # [doc = " `IORING_MSG_SEND_FD`"] SendFd = sys :: io_uring_msg_ring_flags :: IORING_MSG_SEND_FD as _ , }
};
}
