macro_rules! other_407 {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Copy , Clone)] pub union op_flags_union { pub rw_flags : crate :: io :: ReadWriteFlags , pub fsync_flags : IoringFsyncFlags , pub poll_events : u16 , pub poll32_events : u32 , pub sync_range_flags : u32 , # [doc = " `msg_flags` is split into `send_flags` and `recv_flags`."] # [doc (alias = "msg_flags")] pub send_flags : SendFlags , # [doc = " `msg_flags` is split into `send_flags` and `recv_flags`."] # [doc (alias = "msg_flags")] pub recv_flags : RecvFlags , pub timeout_flags : IoringTimeoutFlags , pub accept_flags : SocketFlags , pub cancel_flags : IoringAsyncCancelFlags , pub open_flags : OFlags , pub statx_flags : AtFlags , pub fadvise_advice : Advice , pub splice_flags : SpliceFlags , pub rename_flags : RenameFlags , pub unlink_flags : AtFlags , pub hardlink_flags : AtFlags , pub xattr_flags : XattrFlags , pub msg_ring_flags : IoringMsgringFlags , pub uring_cmd_flags : IoringUringCmdFlags , pub futex_flags : FutexWaitvFlags , pub install_fd_flags : IoringFixedFdFlags , }
    };
}

other_407!();