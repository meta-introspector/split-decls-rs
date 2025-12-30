// Generated macro for impl_743 (impl)
macro_rules! Depcrate_fsimpl_743 {
() => {
// Module: crate::fs
// Provides: {"impl_743"}
// Dependencies: {}
# [stable (feature = "io_traits_arc" , since = "1.73.0")] impl Write for Arc < File > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { (& * * self) . write (buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { (& * * self) . write_vectored (bufs) } # [inline] fn is_write_vectored (& self) -> bool { (& * * self) . is_write_vectored () } # [inline] fn flush (& mut self) -> io :: Result < () > { (& * * self) . flush () } }
};
}
