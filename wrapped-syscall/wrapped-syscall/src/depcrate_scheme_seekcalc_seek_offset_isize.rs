// Generated macro for calc_seek_offset_isize (function)
macro_rules! Depcrate_scheme_seekcalc_seek_offset_isize {
() => {
// Module: crate::scheme::seek
// Provides: {"calc_seek_offset_isize"}
// Dependencies: {}
# [doc = " Helper for seek calls"] # [doc = " Result is guaranteed to be positive."] # [doc = " EOVERFLOW returned if the arguments would cause an overflow."] # [doc = " EINVAL returned if the new offset is out of bounds."] pub fn calc_seek_offset_isize (cur_offset : isize , pos : isize , whence : usize , buf_len : isize ,) -> Result < isize > { let new_offset = match whence { SEEK_CUR => pos . checked_add (cur_offset) , SEEK_END => pos . checked_add (buf_len) , SEEK_SET => Some (pos) , _ => None , } ; match new_offset { Some (new_offset) if new_offset < 0 => Err (Error :: new (EINVAL)) , Some (new_offset) => Ok (cmp :: min (new_offset , buf_len)) , None => Err (Error :: new (EOVERFLOW)) , } }
};
}
