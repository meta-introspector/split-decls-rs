macro_rules! io_uring_files_update {
    () => {
        # [allow (missing_docs)] # [repr (C , align (8))] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct io_uring_files_update { pub offset : u32 , # [doc (hidden)] pub resv : u32 , pub fds : io_uring_ptr , }
    };
}

io_uring_files_update!()