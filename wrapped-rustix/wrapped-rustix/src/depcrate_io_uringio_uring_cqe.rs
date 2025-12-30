// Generated macro for io_uring_cqe (struct)
macro_rules! Depcrate_io_uringio_uring_cqe {
() => {
// Module: crate::io_uring
// Provides: {"io_uring_cqe"}
// Dependencies: {}
# [doc = " An io_uring Completion Queue Entry."] # [doc = ""] # [doc = " This does not derive `Copy` or `Clone` because the `big_cqe` field is not"] # [doc = " automatically copyable."] # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Default)] pub struct io_uring_cqe { pub user_data : io_uring_user_data , pub res : i32 , pub flags : IoringCqeFlags , pub big_cqe : IncompleteArrayField < u64 > , }
};
}
