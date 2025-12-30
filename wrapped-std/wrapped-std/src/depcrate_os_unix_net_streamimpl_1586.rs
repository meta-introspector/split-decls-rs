// Generated macro for impl_1586 (impl)
macro_rules! Depcrate_os_unix_net_streamimpl_1586 {
() => {
// Module: crate::os::unix::net::stream
// Provides: {"impl_1586"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl io :: Read for UnixStream { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { io :: Read :: read (& mut & * self , buf) } fn read_buf (& mut self , buf : io :: BorrowedCursor < '_ >) -> io :: Result < () > { io :: Read :: read_buf (& mut & * self , buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { io :: Read :: read_vectored (& mut & * self , bufs) } # [inline] fn is_read_vectored (& self) -> bool { io :: Read :: is_read_vectored (& & * self) } }
};
}
