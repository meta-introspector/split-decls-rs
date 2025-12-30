// Generated macro for test_deflate_tune (function)
macro_rules! Depcrate_deflatetest_deflate_tune {
() => {
// Module: crate::deflate
// Provides: {"test_deflate_tune"}
// Dependencies: {}
# [test] fn test_deflate_tune () { let mut compr = [0 ; 128] ; let good_length = 3 ; let max_lazy = 5 ; let nice_length = 18 ; let max_chain = 6 ; let mut c_stream = MaybeUninit :: zeroed () ; let err = unsafe { libz_rs_sys :: deflateTune (c_stream . as_mut_ptr () , good_length , max_lazy , nice_length , max_chain ,) } ; assert_eq ! (err , libz_rs_sys :: Z_STREAM_ERROR) ; let err = unsafe { libz_rs_sys :: deflateInit_ (c_stream . as_mut_ptr () , libz_rs_sys :: Z_BEST_COMPRESSION , VERSION , core :: mem :: size_of :: < libz_rs_sys :: z_stream > () as _ ,) } ; assert_eq ! (err , libz_rs_sys :: Z_OK) ; let c_stream = unsafe { c_stream . assume_init_mut () } ; let err = unsafe { libz_rs_sys :: deflateTune (c_stream , good_length , max_lazy , nice_length , max_chain) } ; assert_eq ! (err , libz_rs_sys :: Z_OK) ; let input = "Hello, World!\n" ; c_stream . next_in = input . as_ptr () as * mut u8 ; c_stream . next_out = compr . as_mut_ptr () ; while c_stream . total_in as usize != input . len () && (c_stream . total_out as usize) < compr . len () { c_stream . avail_in = 1 ; c_stream . avail_out = 1 ; let err = unsafe { libz_rs_sys :: deflate (c_stream , Z_NO_FLUSH) } ; assert_eq ! (err , libz_rs_sys :: Z_OK) ; } loop { c_stream . avail_out = 1 ; let err = unsafe { libz_rs_sys :: deflate (c_stream , libz_ng_sys :: Z_FINISH) } ; if err == libz_ng_sys :: Z_STREAM_END { break ; } ; assert_eq ! (err , libz_rs_sys :: Z_OK) ; } let err = unsafe { libz_rs_sys :: deflateEnd (c_stream) } ; assert_eq ! (err , libz_rs_sys :: Z_OK) ; }
};
}
