// Generated macro for impl_1437 (impl)
macro_rules! Depcrate_os_unix_fsimpl_1437 {
() => {
// Module: crate::os::unix::fs
// Provides: {"impl_1437"}
// Dependencies: {}
# [stable (feature = "file_offset" , since = "1.15.0")] impl FileExt for fs :: File { fn read_at (& self , buf : & mut [u8] , offset : u64) -> io :: Result < usize > { self . as_inner () . read_at (buf , offset) } fn read_buf_at (& self , buf : BorrowedCursor < '_ > , offset : u64) -> io :: Result < () > { self . as_inner () . read_buf_at (buf , offset) } fn read_vectored_at (& self , bufs : & mut [io :: IoSliceMut < '_ >] , offset : u64) -> io :: Result < usize > { self . as_inner () . read_vectored_at (bufs , offset) } fn write_at (& self , buf : & [u8] , offset : u64) -> io :: Result < usize > { self . as_inner () . write_at (buf , offset) } fn write_vectored_at (& self , bufs : & [io :: IoSlice < '_ >] , offset : u64) -> io :: Result < usize > { self . as_inner () . write_vectored_at (bufs , offset) } }
};
}
