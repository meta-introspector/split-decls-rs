macro_rules! io_uring_rsrc_update {
    () => {
        # [allow (missing_docs)] # [repr (C , align (8))] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct io_uring_rsrc_update { pub offset : u32 , # [doc (hidden)] pub resv : u32 , pub data : io_uring_ptr , }
    };
}

io_uring_rsrc_update!()