// Generated macro for impl_927 (impl)
macro_rules! Depcrate_io_buffered_linewriterimpl_927 {
() => {
// Module: crate::io::buffered::linewriter
// Provides: {"impl_927"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < W : ? Sized + Write > Write for LineWriter < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { LineWriterShim :: new (& mut self . inner) . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { LineWriterShim :: new (& mut self . inner) . write_vectored (bufs) } fn is_write_vectored (& self) -> bool { self . inner . is_write_vectored () } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { LineWriterShim :: new (& mut self . inner) . write_all (buf) } fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { LineWriterShim :: new (& mut self . inner) . write_all_vectored (bufs) } fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> io :: Result < () > { LineWriterShim :: new (& mut self . inner) . write_fmt (fmt) } }
};
}
