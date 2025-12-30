// Generated macro for vec_write_all (function)
macro_rules! Depcrate_io_cursorvec_write_all {
() => {
// Module: crate::io::cursor
// Provides: {"vec_write_all"}
// Dependencies: {}
# [doc = " Resizing `write_all` implementation for [`Cursor`]."] # [doc = ""] # [doc = " Cursor is allowed to have a pre-allocated and initialised"] # [doc = " vector body, but with a position of 0. This means the [`Write`]"] # [doc = " will overwrite the contents of the vec."] # [doc = ""] # [doc = " This also allows for the vec body to be empty, but with a position of N."] # [doc = " This means that [`Write`] will pad the vec with 0 initially,"] # [doc = " before writing anything from that point"] fn vec_write_all < A > (pos_mut : & mut u64 , vec : & mut Vec < u8 , A > , buf : & [u8]) -> io :: Result < usize > where A : Allocator , { let buf_len = buf . len () ; let mut pos = reserve_and_pad (pos_mut , vec , buf_len) ? ; unsafe { pos = vec_write_all_unchecked (pos , vec , buf) ; if pos > vec . len () { vec . set_len (pos) ; } } ; * pos_mut += buf_len as u64 ; Ok (buf_len) }
};
}
