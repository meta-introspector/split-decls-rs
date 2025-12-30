// Generated macro for chunked_output_rs (function)
macro_rules! Depcrate_inflatechunked_output_rs {
() => {
// Module: crate::inflate
// Provides: {"chunked_output_rs"}
// Dependencies: {}
# [test] fn chunked_output_rs () { use libz_rs_sys :: * ; let input = [99u8 , 96 , 192 , 11 , 24 , 25 , 0] ; let mut stream = MaybeUninit :: < z_stream > :: zeroed () ; let err = unsafe { inflateInit2_ (stream . as_mut_ptr () , - 15 , VERSION , STREAM_SIZE) } ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; let stream = unsafe { stream . assume_init_mut () } ; stream . next_in = input . as_ptr () as * mut u8 ; stream . avail_in = input . len () as _ ; let mut output = [0 ; 33] ; stream . next_out = output . as_mut_ptr () ; stream . avail_out = 32 ; let err = unsafe { inflate (stream , InflateFlush :: NoFlush as _) } ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; assert_eq ! (stream . avail_out , 0) ; assert_eq ! (stream . total_out , 32) ; stream . avail_out = 1 ; let err = unsafe { inflate (stream , InflateFlush :: Finish as _) } ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: StreamEnd) ; unsafe { inflateEnd (stream) } ; assert_eq ! (stream . total_out , 33) ; }
};
}
