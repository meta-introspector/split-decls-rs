// Generated macro for impl_1324 (impl)
macro_rules! Depcrate_net_tcpimpl_1324 {
() => {
// Module: crate::net::tcp
// Provides: {"impl_1324"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Write for & TcpStream { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . 0 . write_vectored (bufs) } # [inline] fn is_write_vectored (& self) -> bool { self . 0 . is_write_vectored () } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
