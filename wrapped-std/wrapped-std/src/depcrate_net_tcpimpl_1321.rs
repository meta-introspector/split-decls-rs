// Generated macro for impl_1321 (impl)
macro_rules! Depcrate_net_tcpimpl_1321 {
() => {
// Module: crate::net::tcp
// Provides: {"impl_1321"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Read for TcpStream { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . read (buf) } fn read_buf (& mut self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 . read_buf (buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 . read_vectored (bufs) } # [inline] fn is_read_vectored (& self) -> bool { self . 0 . is_read_vectored () } }
};
}
