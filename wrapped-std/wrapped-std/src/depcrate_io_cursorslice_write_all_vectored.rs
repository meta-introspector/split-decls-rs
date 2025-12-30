// Generated macro for slice_write_all_vectored (function)
macro_rules! Depcrate_io_cursorslice_write_all_vectored {
() => {
// Module: crate::io::cursor
// Provides: {"slice_write_all_vectored"}
// Dependencies: {}
# [inline] fn slice_write_all_vectored (pos_mut : & mut u64 , slice : & mut [u8] , bufs : & [IoSlice < '_ >] ,) -> io :: Result < () > { for buf in bufs { let n = slice_write (pos_mut , slice , buf) ? ; if n < buf . len () { return Err (io :: Error :: WRITE_ALL_EOF) ; } } Ok (()) }
};
}
