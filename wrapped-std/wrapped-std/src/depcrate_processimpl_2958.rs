// Generated macro for impl_2958 (impl)
macro_rules! Depcrate_processimpl_2958 {
() => {
// Module: crate::process
// Provides: {"impl_2958"}
// Dependencies: {}
# [stable (feature = "process" , since = "1.0.0")] impl Write for ChildStdin { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { (& * self) . write (buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { (& * self) . write_vectored (bufs) } fn is_write_vectored (& self) -> bool { io :: Write :: is_write_vectored (& & * self) } # [inline] fn flush (& mut self) -> io :: Result < () > { (& * self) . flush () } }
};
}
