// Generated macro for impl_1163 (impl)
macro_rules! Depcrate_io_stdioimpl_1163 {
() => {
// Module: crate::io::stdio
// Provides: {"impl_1163"}
// Dependencies: {}
# [stable (feature = "write_mt" , since = "1.48.0")] impl Write for & Stdout { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . lock () . write (buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . lock () . write_vectored (bufs) } # [inline] fn is_write_vectored (& self) -> bool { self . lock () . is_write_vectored () } fn flush (& mut self) -> io :: Result < () > { self . lock () . flush () } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . lock () . write_all (buf) } fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { self . lock () . write_all_vectored (bufs) } fn write_fmt (& mut self , args : fmt :: Arguments < '_ >) -> io :: Result < () > { self . lock () . write_fmt (args) } }
};
}
