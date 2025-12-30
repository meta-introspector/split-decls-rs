// Generated macro for vec_write_all_unchecked (function)
macro_rules! Depcrate_io_cursorvec_write_all_unchecked {
() => {
// Module: crate::io::cursor
// Provides: {"vec_write_all_unchecked"}
// Dependencies: {}
# [doc = " Writes the slice to the vec without allocating."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `vec` must have `buf.len()` spare capacity."] unsafe fn vec_write_all_unchecked < A > (pos : usize , vec : & mut Vec < u8 , A > , buf : & [u8]) -> usize where A : Allocator , { debug_assert ! (vec . capacity () >= pos + buf . len ()) ; unsafe { vec . as_mut_ptr () . add (pos) . copy_from (buf . as_ptr () , buf . len ()) } ; pos + buf . len () }
};
}
