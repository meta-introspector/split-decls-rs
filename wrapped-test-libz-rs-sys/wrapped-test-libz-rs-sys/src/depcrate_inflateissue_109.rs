// Generated macro for issue_109 (function)
macro_rules! Depcrate_inflateissue_109 {
() => {
// Module: crate::inflate
// Provides: {"issue_109"}
// Dependencies: {}
# [test] fn issue_109 () { let input = & include_bytes ! ("test-data/issue-109.gz") [10 ..] [.. 32758] ; let window_bits = - 15 ; assert_eq_rs_ng ! ({ let mut output : Vec < u8 > = Vec :: with_capacity (1 << 15) ; let mut buf = [0 ; 8192] ; let mut stream = MaybeUninit ::< z_stream >:: zeroed () ; let err = unsafe { inflateInit2_ (stream . as_mut_ptr () , window_bits , zlibVersion () , core :: mem :: size_of ::< z_stream > () as c_int ,) } ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; let stream = unsafe { stream . assume_init_mut () } ; stream . next_in = input . as_ptr () as * mut u8 ; stream . avail_in = input . len () as _ ; while stream . avail_in != 0 { stream . next_out = buf . as_mut_ptr () ; stream . avail_out = buf . len () as _ ; let err = unsafe { inflate (stream , InflateFlush :: NoFlush as _) } ; if ReturnCode :: from (err) == ReturnCode :: BufError { output . extend (& buf [.. stream . avail_out as usize]) ; stream . avail_out = buf . len () as _ ; continue ; } } let err = inflateEnd (stream) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; output }) ; }
};
}
