// Generated macro for io_uring_restriction (struct)
macro_rules! Depcrate_io_uringio_uring_restriction {
() => {
// Module: crate::io_uring
// Provides: {"io_uring_restriction"}
// Dependencies: {}
# [allow (missing_docs)] # [repr (C)] # [derive (Copy , Clone , Default)] # [non_exhaustive] pub struct io_uring_restriction { pub opcode : IoringRestrictionOp , pub register_or_sqe_op_or_sqe_flags : register_or_sqe_op_or_sqe_flags_union , # [doc (hidden)] pub resv : u8 , # [doc (hidden)] pub resv2 : [u32 ; 3] , }
};
}
