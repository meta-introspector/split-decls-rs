// Generated macro for impl_1138 (impl)
macro_rules! Depcrate_io_stdioimpl_1138 {
() => {
// Module: crate::io::stdio
// Provides: {"impl_1138"}
// Dependencies: {}
impl Write for StdoutRaw { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { handle_ebadf (self . 0 . write (buf) , | | Ok (buf . len ())) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let total = | | Ok (bufs . iter () . map (| b | b . len ()) . sum ()) ; handle_ebadf (self . 0 . write_vectored (bufs) , total) } # [inline] fn is_write_vectored (& self) -> bool { self . 0 . is_write_vectored () } fn flush (& mut self) -> io :: Result < () > { handle_ebadf (self . 0 . flush () , | | Ok (())) } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { handle_ebadf (self . 0 . write_all (buf) , | | Ok (())) } fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { handle_ebadf (self . 0 . write_all_vectored (bufs) , | | Ok (())) } fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> io :: Result < () > { handle_ebadf (self . 0 . write_fmt (fmt) , | | Ok (())) } }
};
}
