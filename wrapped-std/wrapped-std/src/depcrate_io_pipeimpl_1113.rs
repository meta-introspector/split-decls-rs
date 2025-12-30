// Generated macro for impl_1113 (impl)
macro_rules! Depcrate_io_pipeimpl_1113 {
() => {
// Module: crate::io::pipe
// Provides: {"impl_1113"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] impl io :: Write for PipeWriter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { self . 0 . write_vectored (bufs) } # [inline] fn is_write_vectored (& self) -> bool { self . 0 . is_write_vectored () } }
};
}
