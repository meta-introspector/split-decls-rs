macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! io_uring_sync_cancel_reg {
    () => {
        deps!();
        # [allow (missing_docs)] # [repr (C)] # [derive (Copy , Clone)] # [non_exhaustive] pub struct io_uring_sync_cancel_reg { pub addr : io_uring_user_data , pub fd : i32 , pub flags : IoringAsyncCancelFlags , pub timeout : Timespec , pub opcode : u8 , # [doc (hidden)] pub pad : [u8 ; 7] , # [doc (hidden)] pub pad2 : [u64 ; 3] , }
    };
}

io_uring_sync_cancel_reg!();