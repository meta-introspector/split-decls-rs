// Generated macro for other_651 (other)
macro_rules! Depcrate_io_uringother_651 {
() => {
// Module: crate::io_uring
// Provides: {"other_651"}
// Dependencies: {}
# [allow (missing_docs)] # [repr (C)] # [derive (Copy , Clone)] pub union addr_or_splice_off_in_union { pub addr : io_uring_ptr , pub splice_off_in : u64 , pub msgring_cmd : IoringMsgringCmds , pub user_data : io_uring_user_data , }
};
}
