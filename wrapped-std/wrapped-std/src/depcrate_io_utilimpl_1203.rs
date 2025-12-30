// Generated macro for impl_1203 (impl)
macro_rules! Depcrate_io_utilimpl_1203 {
() => {
// Module: crate::io::util
// Provides: {"impl_1203"}
// Dependencies: {}
# [stable (feature = "empty_write" , since = "1.73.0")] impl Write for & Empty { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { Ok (buf . len ()) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let total_len = bufs . iter () . map (| b | b . len ()) . sum () ; Ok (total_len) } # [inline] fn is_write_vectored (& self) -> bool { true } # [inline] fn write_all (& mut self , _buf : & [u8]) -> io :: Result < () > { Ok (()) } # [inline] fn write_all_vectored (& mut self , _bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { Ok (()) } # [inline] fn write_fmt (& mut self , _args : fmt :: Arguments < '_ >) -> io :: Result < () > { Ok (()) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
