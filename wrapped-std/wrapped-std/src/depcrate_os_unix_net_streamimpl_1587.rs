// Generated macro for impl_1587 (impl)
macro_rules! Depcrate_os_unix_net_streamimpl_1587 {
() => {
// Module: crate::os::unix::net::stream
// Provides: {"impl_1587"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl < 'a > io :: Read for & 'a UnixStream { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . read (buf) } fn read_buf (& mut self , buf : io :: BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 . read_buf (buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 . read_vectored (bufs) } # [inline] fn is_read_vectored (& self) -> bool { self . 0 . is_read_vectored () } }
};
}
