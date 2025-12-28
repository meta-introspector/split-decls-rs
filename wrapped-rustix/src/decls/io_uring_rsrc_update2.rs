macro_rules! io_uring_rsrc_update2 {
    () => {
        # [allow (missing_docs)] # [repr (C , align (8))] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct io_uring_rsrc_update2 { pub offset : u32 , # [doc (hidden)] pub resv : u32 , pub data : io_uring_ptr , pub tags : io_uring_ptr , pub nr : u32 , # [doc (hidden)] pub resv2 : u32 , }
    };
}

io_uring_rsrc_update2!()