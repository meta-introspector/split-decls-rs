// Generated macro for test_compress_param (function)
macro_rules! Depcrate_deflatetest_compress_param {
() => {
// Module: crate::deflate
// Provides: {"test_compress_param"}
// Dependencies: {}
# [test] fn test_compress_param () { let config = DeflateConfig :: new (2) ; let input = "Scheduling and executing async tasks is a job handled by an async runtime, such as\0" ; assert_eq_rs_ng ! ({ let mut output = [0 ; 1024] ; let mut strm = MaybeUninit :: zeroed () ; let err = deflateInit2_ (strm . as_mut_ptr () , config . level , config . method as i32 , config . window_bits , config . mem_level , config . strategy as i32 , VERSION , STREAM_SIZE ,) ; assert_eq ! (err , 0) ; let stream = strm . assume_init_mut () ; stream . next_out = output . as_mut_ptr () ; stream . avail_out = output . len () as _ ; let offset = input . len () / 2 ; stream . next_in = input . as_ptr () as * mut u8 ; stream . avail_in = offset as _ ; let err = deflate (stream , DeflateFlush :: NoFlush as i32) ; assert_eq ! (err , 0) ; let err = deflateParams (stream , 8 , Strategy :: Rle as i32) ; assert_eq ! (err , 0) ; assert_eq ! (stream . next_in as usize - input . as_ptr () as usize , offset) ; stream . avail_in = (input . len () - offset) as _ ; let err = deflate (stream , DeflateFlush :: Finish as i32) ; assert_eq ! (err , ReturnCode :: StreamEnd as i32) ; let err = deflateEnd (stream) ; assert_eq ! (err , 0) ; output [.. stream . total_out as usize] . to_vec () }) ; }
};
}
