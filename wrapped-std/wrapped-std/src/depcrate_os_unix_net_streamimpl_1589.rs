// Generated macro for impl_1589 (impl)
macro_rules! Depcrate_os_unix_net_streamimpl_1589 {
() => {
// Module: crate::os::unix::net::stream
// Provides: {"impl_1589"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl < 'a > io :: Write for & 'a UnixStream { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . send_with_flags (buf , MSG_NOSIGNAL) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . 0 . write_vectored (bufs) } # [inline] fn is_write_vectored (& self) -> bool { self . 0 . is_write_vectored () } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
