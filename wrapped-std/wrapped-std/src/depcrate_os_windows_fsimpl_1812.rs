// Generated macro for impl_1812 (impl)
macro_rules! Depcrate_os_windows_fsimpl_1812 {
() => {
// Module: crate::os::windows::fs
// Provides: {"impl_1812"}
// Dependencies: {}
# [stable (feature = "file_offset" , since = "1.15.0")] impl FileExt for fs :: File { fn seek_read (& self , buf : & mut [u8] , offset : u64) -> io :: Result < usize > { self . as_inner () . read_at (buf , offset) } fn seek_read_buf (& self , buf : BorrowedCursor < '_ > , offset : u64) -> io :: Result < () > { self . as_inner () . read_buf_at (buf , offset) } fn seek_write (& self , buf : & [u8] , offset : u64) -> io :: Result < usize > { self . as_inner () . write_at (buf , offset) } }
};
}
