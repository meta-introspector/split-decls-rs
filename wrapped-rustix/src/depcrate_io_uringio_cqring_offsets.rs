// Generated macro for io_cqring_offsets (struct)
macro_rules! Depcrate_io_uringio_cqring_offsets {
() => {
// Module: crate::io_uring
// Provides: {"io_cqring_offsets"}
// Dependencies: {}
# [allow (missing_docs)] # [repr (C)] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct io_cqring_offsets { pub head : u32 , pub tail : u32 , pub ring_mask : u32 , pub ring_entries : u32 , pub overflow : u32 , pub cqes : u32 , pub flags : u32 , # [doc (hidden)] pub resv1 : u32 , pub user_addr : io_uring_ptr , }
};
}
