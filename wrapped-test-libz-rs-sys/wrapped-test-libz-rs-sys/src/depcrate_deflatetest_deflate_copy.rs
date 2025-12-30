// Generated macro for test_deflate_copy (function)
macro_rules! Depcrate_deflatetest_deflate_copy {
() => {
// Module: crate::deflate
// Provides: {"test_deflate_copy"}
// Dependencies: {}
# [test] fn test_deflate_copy () { const HELLO : & str = "hello, hello!\0" ; let config = DeflateConfig :: default () ; let mut strm = MaybeUninit :: zeroed () ; unsafe { let err = libz_rs_sys :: deflateInit2_ (strm . as_mut_ptr () , config . level , config . method as i32 , config . window_bits , config . mem_level , config . strategy as i32 , VERSION , STREAM_SIZE ,) ; assert_eq ! (err , 0) ; let strm = strm . assume_init_mut () ; let mut compr = [0 ; 32] ; strm . next_in = HELLO . as_ptr () as * mut u8 ; strm . next_out = compr . as_mut_ptr () ; for _ in HELLO . as_bytes () { strm . avail_in = 1 ; strm . avail_out = 1 ; let err = libz_rs_sys :: deflate (strm , DeflateFlush :: NoFlush as i32) ; assert_eq ! (err , 0) ; } loop { strm . avail_out = 1 ; let err = libz_rs_sys :: deflate (strm , DeflateFlush :: Finish as i32) ; if ReturnCode :: from (err) == ReturnCode :: StreamEnd { break ; } assert_eq ! (err , 0) ; } let mut copy = MaybeUninit :: uninit () ; let err = libz_rs_sys :: deflateCopy (copy . as_mut_ptr () , strm) ; assert_eq ! (err , 0) ; assert_eq ! (* (strm . state as * const u8) , * (((* copy . as_mut_ptr ()) . state) as * const u8) ,) ; let err = libz_rs_sys :: deflateEnd (strm) ; assert_eq ! (err , 0) ; let err = libz_rs_sys :: deflateEnd (copy . as_mut_ptr ()) ; assert_eq ! (err , 0) ; } }
};
}
