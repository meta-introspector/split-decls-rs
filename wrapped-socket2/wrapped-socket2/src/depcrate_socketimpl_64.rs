// Generated macro for impl_64 (impl)
macro_rules! Depcrate_socketimpl_64 {
() => {
// Module: crate::socket
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a > Write for & 'a Socket { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . send (buf) } # [cfg (not (target_os = "redox"))] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . send_vectored (bufs) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
