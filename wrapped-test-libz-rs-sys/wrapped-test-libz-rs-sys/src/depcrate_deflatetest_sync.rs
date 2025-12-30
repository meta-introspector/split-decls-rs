// Generated macro for test_sync (function)
macro_rules! Depcrate_deflatetest_sync {
() => {
// Module: crate::deflate
// Provides: {"test_sync"}
// Dependencies: {}
fn test_sync (compr : & [u8]) { let mut uncompr = [0xAA ; 32] ; let mut stream = MaybeUninit :: zeroed () ; let config = InflateConfig :: default () ; unsafe { let err = libz_rs_sys :: inflateInit2_ (stream . as_mut_ptr () , config . window_bits , VERSION , STREAM_SIZE ,) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; let stream = stream . assume_init_mut () ; stream . next_in = compr . as_ptr () as * mut u8 ; stream . avail_in = 2 ; stream . next_out = uncompr . as_mut_ptr () ; stream . avail_out = uncompr . len () as _ ; let err = libz_rs_sys :: inflate (stream , DeflateFlush :: NoFlush as i32) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; stream . avail_in = (compr . len () - 2) as _ ; let err = libz_rs_sys :: inflateSync (stream) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; let err = libz_rs_sys :: inflate (stream , DeflateFlush :: Finish as i32) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: StreamEnd) ; let err = libz_rs_sys :: inflateEnd (stream) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; } }
};
}
