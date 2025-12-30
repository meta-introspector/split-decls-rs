// Generated macro for test_deflate_pending (function)
macro_rules! Depcrate_deflatetest_deflate_pending {
() => {
// Module: crate::deflate
// Provides: {"test_deflate_pending"}
// Dependencies: {}
# [test] fn test_deflate_pending () { const HELLO : & str = "hello, hello!\0" ; let config = DeflateConfig :: default () ; let mut strm = MaybeUninit :: zeroed () ; unsafe { let err = libz_rs_sys :: deflateInit2_ (strm . as_mut_ptr () , config . level , config . method as i32 , config . window_bits , config . mem_level , config . strategy as i32 , VERSION , STREAM_SIZE ,) ; assert_eq ! (err , 0) ; let strm = strm . assume_init_mut () ; let mut compr = [0 ; 32] ; strm . next_in = HELLO . as_ptr () as * mut u8 ; strm . next_out = compr . as_mut_ptr () ; let it_in = HELLO . as_bytes () . chunks (1) ; let mut it_out = compr . chunks (1) ; for (_ , _) in it_in . zip (& mut it_out) { strm . avail_in = 1 ; strm . avail_out = 1 ; let err = deflate (strm , DeflateFlush :: NoFlush as i32) ; assert_eq ! (err , 0) ; } let mut ped = 0 ; let mut bits = 0 ; let err = libz_rs_sys :: deflatePending (strm , std :: ptr :: null_mut () , std :: ptr :: null_mut ()) ; assert_eq ! (err , 0) ; let err = libz_rs_sys :: deflatePending (strm , & mut ped , std :: ptr :: null_mut ()) ; assert_eq ! (err , 0) ; let err = libz_rs_sys :: deflatePending (strm , std :: ptr :: null_mut () , & mut bits) ; assert_eq ! (err , 0) ; let err = libz_rs_sys :: deflatePending (strm , & mut ped , & mut bits) ; assert_eq ! (err , 0) ; assert ! (bits >= 0) ; assert ! (bits <= 7) ; for _ in it_out { strm . avail_out = 1 ; let err = deflate (strm , libz_rs_sys :: Z_FINISH) ; if err == libz_rs_sys :: Z_STREAM_END { break ; } ; assert_eq ! (err , 0) ; } let err = deflateEnd (strm) ; assert_eq ! (err , 0) ; } }
};
}
