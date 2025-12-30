// Generated macro for calc_seek_offset_usize (function)
macro_rules! Depcrate_scheme_seekcalc_seek_offset_usize {
() => {
// Module: crate::scheme::seek
// Provides: {"calc_seek_offset_usize"}
// Dependencies: {}
# [doc = " Helper for seek calls"] # [doc = " In most cases it's easier to use a usize to track the offset and buffer size internally,"] # [doc = " but the seek interface uses isize.  This wrapper ensures EOVERFLOW errors are returned"] # [doc = " as appropriate if the value in the usize can't fit in the isize."] pub fn calc_seek_offset_usize (cur_offset : usize , pos : isize , whence : usize , buf_len : usize ,) -> Result < isize > { let cur_offset = isize :: try_from (cur_offset) . or_else (| _ | Err (Error :: new (EOVERFLOW))) ? ; let buf_len = isize :: try_from (buf_len) . or_else (| _ | Err (Error :: new (EOVERFLOW))) ? ; calc_seek_offset_isize (cur_offset , pos , whence , buf_len) }
};
}
