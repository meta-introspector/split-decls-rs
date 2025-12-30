// Generated macro for impl_2959 (impl)
macro_rules! Depcrate_processimpl_2959 {
() => {
// Module: crate::process
// Provides: {"impl_2959"}
// Dependencies: {}
# [stable (feature = "write_mt" , since = "1.48.0")] impl Write for & ChildStdin { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . inner . write (buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . inner . write_vectored (bufs) } fn is_write_vectored (& self) -> bool { self . inner . is_write_vectored () } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
