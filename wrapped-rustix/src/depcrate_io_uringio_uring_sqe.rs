// Generated macro for io_uring_sqe (struct)
macro_rules! Depcrate_io_uringio_uring_sqe {
() => {
// Module: crate::io_uring
// Provides: {"io_uring_sqe"}
// Dependencies: {}
# [doc = " An io_uring Submission Queue Entry."] # [allow (missing_docs)] # [repr (C)] # [derive (Copy , Clone , Default)] pub struct io_uring_sqe { pub opcode : IoringOp , pub flags : IoringSqeFlags , pub ioprio : ioprio_union , pub fd : RawFd , pub off_or_addr2 : off_or_addr2_union , pub addr_or_splice_off_in : addr_or_splice_off_in_union , pub len : len_union , pub op_flags : op_flags_union , pub user_data : io_uring_user_data , pub buf : buf_union , pub personality : u16 , pub splice_fd_in_or_file_index_or_addr_len : splice_fd_in_or_file_index_or_addr_len_union , pub addr3_or_cmd : addr3_or_cmd_union , }
};
}
