// Generated macro for slice_write_all (function)
macro_rules! Depcrate_io_cursorslice_write_all {
() => {
// Module: crate::io::cursor
// Provides: {"slice_write_all"}
// Dependencies: {}
# [inline] fn slice_write_all (pos_mut : & mut u64 , slice : & mut [u8] , buf : & [u8]) -> io :: Result < () > { let n = slice_write (pos_mut , slice , buf) ? ; if n < buf . len () { Err (io :: Error :: WRITE_ALL_EOF) } else { Ok (()) } }
};
}
