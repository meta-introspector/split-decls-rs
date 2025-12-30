// Generated macro for read_buf_window (function)
macro_rules! Depcrate_deflateread_buf_window {
() => {
// Module: crate::deflate
// Provides: {"read_buf_window"}
// Dependencies: {}
pub (crate) fn read_buf_window (stream : & mut DeflateStream , offset : usize , size : usize) -> usize { let len = Ord :: min (stream . avail_in as usize , size) ; if len == 0 { return 0 ; } stream . avail_in -= len as u32 ; if stream . state . wrap == 2 { let window = & mut stream . state . window ; unsafe { window . copy_and_initialize (offset .. offset + len , stream . next_in) } ; let data = & stream . state . window . filled () [offset ..] [.. len] ; stream . state . crc_fold . fold (data , CRC32_INITIAL_VALUE) ; } else if stream . state . wrap == 1 { let window = & mut stream . state . window ; unsafe { window . copy_and_initialize (offset .. offset + len , stream . next_in) } ; let data = & stream . state . window . filled () [offset ..] [.. len] ; stream . adler = adler32 (stream . adler as u32 , data) as _ ; } else { let window = & mut stream . state . window ; unsafe { window . copy_and_initialize (offset .. offset + len , stream . next_in) } ; } stream . next_in = stream . next_in . wrapping_add (len) ; stream . total_in += len as crate :: c_api :: z_size ; len }
};
}
