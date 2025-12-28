macro_rules! io_uring_buf {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct io_uring_buf { pub addr : io_uring_ptr , pub len : u32 , pub bid : u16 , # [doc (hidden)] pub resv : u16 , }
    };
}

io_uring_buf!();