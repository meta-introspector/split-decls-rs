// Generated macro for impl_1175 (impl)
macro_rules! Depcrate_io_stdioimpl_1175 {
() => {
// Module: crate::io::stdio
// Provides: {"impl_1175"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Write for Stderr { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { (& * self) . write (buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { (& * self) . write_vectored (bufs) } # [inline] fn is_write_vectored (& self) -> bool { io :: Write :: is_write_vectored (& & * self) } fn flush (& mut self) -> io :: Result < () > { (& * self) . flush () } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { (& * self) . write_all (buf) } fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { (& * self) . write_all_vectored (bufs) } fn write_fmt (& mut self , args : fmt :: Arguments < '_ >) -> io :: Result < () > { (& * self) . write_fmt (args) } }
};
}
