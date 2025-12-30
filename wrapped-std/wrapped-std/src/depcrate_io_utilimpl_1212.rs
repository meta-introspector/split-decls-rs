// Generated macro for impl_1212 (impl)
macro_rules! Depcrate_io_utilimpl_1212 {
() => {
// Module: crate::io::util
// Provides: {"impl_1212"}
// Dependencies: {}
# [stable (feature = "write_mt" , since = "1.48.0")] impl Write for & Sink { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { Ok (buf . len ()) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let total_len = bufs . iter () . map (| b | b . len ()) . sum () ; Ok (total_len) } # [inline] fn is_write_vectored (& self) -> bool { true } # [inline] fn write_all (& mut self , _buf : & [u8]) -> io :: Result < () > { Ok (()) } # [inline] fn write_all_vectored (& mut self , _bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { Ok (()) } # [inline] fn write_fmt (& mut self , _args : fmt :: Arguments < '_ >) -> io :: Result < () > { Ok (()) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
