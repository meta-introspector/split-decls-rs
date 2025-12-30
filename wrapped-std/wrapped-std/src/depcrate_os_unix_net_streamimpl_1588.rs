// Generated macro for impl_1588 (impl)
macro_rules! Depcrate_os_unix_net_streamimpl_1588 {
() => {
// Module: crate::os::unix::net::stream
// Provides: {"impl_1588"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl io :: Write for UnixStream { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { io :: Write :: write (& mut & * self , buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { io :: Write :: write_vectored (& mut & * self , bufs) } # [inline] fn is_write_vectored (& self) -> bool { io :: Write :: is_write_vectored (& & * self) } fn flush (& mut self) -> io :: Result < () > { io :: Write :: flush (& mut & * self) } }
};
}
