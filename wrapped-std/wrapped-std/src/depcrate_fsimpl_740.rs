// Generated macro for impl_740 (impl)
macro_rules! Depcrate_fsimpl_740 {
() => {
// Module: crate::fs
// Provides: {"impl_740"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Write for File { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { (& * self) . write (buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { (& * self) . write_vectored (bufs) } # [inline] fn is_write_vectored (& self) -> bool { (& & * self) . is_write_vectored () } # [inline] fn flush (& mut self) -> io :: Result < () > { (& * self) . flush () } }
};
}
