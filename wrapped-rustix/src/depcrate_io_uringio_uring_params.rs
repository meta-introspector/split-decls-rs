// Generated macro for io_uring_params (struct)
macro_rules! Depcrate_io_uringio_uring_params {
() => {
// Module: crate::io_uring
// Provides: {"io_uring_params"}
// Dependencies: {}
# [allow (missing_docs)] # [repr (C)] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct io_uring_params { pub sq_entries : u32 , pub cq_entries : u32 , pub flags : IoringSetupFlags , pub sq_thread_cpu : u32 , pub sq_thread_idle : u32 , pub features : IoringFeatureFlags , pub wq_fd : RawFd , # [doc (hidden)] pub resv : [u32 ; 3] , pub sq_off : io_sqring_offsets , pub cq_off : io_cqring_offsets , }
};
}
