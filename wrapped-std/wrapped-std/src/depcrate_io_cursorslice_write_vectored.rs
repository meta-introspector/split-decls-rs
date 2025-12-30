// Generated macro for slice_write_vectored (function)
macro_rules! Depcrate_io_cursorslice_write_vectored {
() => {
// Module: crate::io::cursor
// Provides: {"slice_write_vectored"}
// Dependencies: {}
# [inline] fn slice_write_vectored (pos_mut : & mut u64 , slice : & mut [u8] , bufs : & [IoSlice < '_ >] ,) -> io :: Result < usize > { let mut nwritten = 0 ; for buf in bufs { let n = slice_write (pos_mut , slice , buf) ? ; nwritten += n ; if n < buf . len () { break ; } } Ok (nwritten) }
};
}
