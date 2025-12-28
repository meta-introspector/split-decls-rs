macro_rules! io_uring_clone_buffers {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Default)] # [non_exhaustive] pub struct io_uring_clone_buffers { pub src_fd : u32 , pub flags : u32 , pub src_off : u32 , pub dst_off : u32 , pub nr : u32 , # [doc (hidden)] pub pad : [u32 ; 3] , }
    };
}

io_uring_clone_buffers!();