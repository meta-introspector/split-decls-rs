macro_rules! io_uring_napi {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Default)] # [non_exhaustive] pub struct io_uring_napi { pub busy_poll_to : u32 , pub prefer_busy_poll : u8 , pub opcode : u8 , # [doc (hidden)] pub pad : [u8 ; 2] , pub op_param : u32 , # [doc (hidden)] pub resv : u32 , }
    };
}

io_uring_napi!();