// Generated macro for tail_or_bufs_struct (struct)
macro_rules! Depcrate_io_uringtail_or_bufs_struct {
() => {
// Module: crate::io_uring
// Provides: {"tail_or_bufs_struct"}
// Dependencies: {}
# [allow (missing_docs)] # [repr (C)] # [derive (Debug , Default)] pub struct tail_or_bufs_struct { pub tail : UnionField < buf_ring_tail_struct > , pub bufs : UnionField < buf_ring_bufs_struct > , pub union_field : [u64 ; 2] , }
};
}
