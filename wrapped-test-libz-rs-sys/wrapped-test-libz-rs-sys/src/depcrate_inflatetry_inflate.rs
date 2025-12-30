// Generated macro for try_inflate (function)
macro_rules! Depcrate_inflatetry_inflate {
() => {
// Module: crate::inflate
// Provides: {"try_inflate"}
// Dependencies: {}
fn try_inflate (input : & [u8] , err : c_int) -> c_int { use libz_rs_sys :: * ; let len = input . len () ; let size = len << 3 ; let mut out = vec ! [0 ; size] ; let mut strm = mem_setup () ; strm . avail_in = 0 ; strm . next_in = std :: ptr :: null_mut () ; let mut ret = unsafe { inflateInit2_ (& mut strm , if err < 0 { 47 } else { - 15 } , VERSION , STREAM_SIZE ,) } ; assert_eq ! (ret , Z_OK) ; strm . avail_in = len as _ ; strm . next_in = input . as_ptr () as * mut u8 ; loop { strm . avail_out = size as _ ; strm . next_out = out . as_mut_ptr () ; ret = unsafe { inflate (& mut strm , InflateFlush :: Trees as _) } ; assert ! (! matches ! (ret , Z_STREAM_ERROR | Z_MEM_ERROR)) ; if matches ! (ret , Z_DATA_ERROR | Z_NEED_DICT) { break ; } if ! (strm . avail_in > 0 || strm . avail_out == 0) { break ; } } if err != Z_OK { assert_eq ! (ret , Z_DATA_ERROR) ; } unsafe { inflateEnd (& mut strm) } ; mem_done (& mut strm) ; ret }
};
}
