// Generated macro for io_uring_files_update (struct)
macro_rules! Depcrate_io_uringio_uring_files_update {
() => {
// Module: crate::io_uring
// Provides: {"io_uring_files_update"}
// Dependencies: {}
# [allow (missing_docs)] # [repr (C , align (8))] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct io_uring_files_update { pub offset : u32 , # [doc (hidden)] pub resv : u32 , pub fds : io_uring_ptr , }
};
}
