// Generated macro for slice_write (function)
macro_rules! Depcrate_io_cursorslice_write {
() => {
// Module: crate::io::cursor
// Provides: {"slice_write"}
// Dependencies: {}
# [inline] fn slice_write (pos_mut : & mut u64 , slice : & mut [u8] , buf : & [u8]) -> io :: Result < usize > { let pos = cmp :: min (* pos_mut , slice . len () as u64) ; let amt = (& mut slice [(pos as usize) ..]) . write (buf) ? ; * pos_mut += amt as u64 ; Ok (amt) }
};
}
