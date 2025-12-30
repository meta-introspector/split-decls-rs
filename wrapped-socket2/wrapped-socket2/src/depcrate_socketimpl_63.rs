// Generated macro for impl_63 (impl)
macro_rules! Depcrate_socketimpl_63 {
() => {
// Module: crate::socket
// Provides: {"impl_63"}
// Dependencies: {}
impl Write for Socket { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . send (buf) } # [cfg (not (target_os = "redox"))] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . send_vectored (bufs) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
