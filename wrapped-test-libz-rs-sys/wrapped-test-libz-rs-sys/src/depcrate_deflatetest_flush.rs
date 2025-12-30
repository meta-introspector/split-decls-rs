// Generated macro for test_flush (function)
macro_rules! Depcrate_deflatetest_flush {
() => {
// Module: crate::deflate
// Provides: {"test_flush"}
// Dependencies: {}
# [doc = " test deflate() with DeflateFlush::Full"] # [test] fn test_flush () { let config = DeflateConfig :: default () ; const HELLO : & str = "hello, hello!\0" ; let mut compr = [0 ; 32] ; unsafe { let mut strm = MaybeUninit :: zeroed () ; let err = libz_rs_sys :: deflateInit2_ (strm . as_mut_ptr () , config . level , config . method as i32 , config . window_bits , config . mem_level , config . strategy as i32 , VERSION , STREAM_SIZE ,) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; let stream = strm . assume_init_mut () ; stream . next_in = HELLO . as_ptr () as * mut u8 ; stream . next_out = compr . as_mut_ptr () ; stream . avail_in = 3 ; stream . avail_out = compr . len () as _ ; let err = libz_rs_sys :: deflate (stream , DeflateFlush :: FullFlush as i32) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; compr [3] += 1 ; stream . avail_in = (HELLO . len () - 3) as _ ; let err = libz_rs_sys :: deflate (stream , DeflateFlush :: Finish as i32) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: StreamEnd) ; let err = libz_rs_sys :: deflateEnd (stream) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; } test_sync (& compr) }
};
}
