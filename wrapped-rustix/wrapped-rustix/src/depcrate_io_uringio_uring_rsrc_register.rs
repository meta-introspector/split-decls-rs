// Generated macro for io_uring_rsrc_register (struct)
macro_rules! Depcrate_io_uringio_uring_rsrc_register {
() => {
// Module: crate::io_uring
// Provides: {"io_uring_rsrc_register"}
// Dependencies: {}
# [allow (missing_docs)] # [repr (C , align (8))] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct io_uring_rsrc_register { pub nr : u32 , pub flags : IoringRsrcFlags , # [doc (hidden)] pub resv2 : u64 , pub data : io_uring_ptr , pub tags : io_uring_ptr , }
};
}
