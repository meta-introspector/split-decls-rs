macro_rules! io_uring_buf_reg {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct io_uring_buf_reg { pub ring_addr : io_uring_ptr , pub ring_entries : u32 , pub bgid : u16 , pub flags : u16 , # [doc (hidden)] pub resv : [u64 ; 3_usize] , }
    };
}

io_uring_buf_reg!()