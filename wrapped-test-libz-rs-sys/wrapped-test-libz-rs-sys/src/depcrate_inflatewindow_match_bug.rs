// Generated macro for window_match_bug (function)
macro_rules! Depcrate_inflatewindow_match_bug {
() => {
// Module: crate::inflate
// Provides: {"window_match_bug"}
// Dependencies: {}
# [test] fn window_match_bug () { let input = & include_bytes ! ("test-data/window-match-bug.zraw") ; let window_bits = - 10 ; assert_eq_rs_ng ! ({ let mut output : Vec < u8 > = Vec :: with_capacity (1 << 15) ; let mut buf = [0 ; 402] ; let mut stream = MaybeUninit ::< z_stream >:: zeroed () ; let err = unsafe { inflateInit2_ (stream . as_mut_ptr () , window_bits , zlibVersion () , core :: mem :: size_of ::< z_stream > () as c_int ,) } ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; let stream = unsafe { stream . assume_init_mut () } ; stream . next_in = input . as_ptr () as * mut u8 ; stream . avail_in = input . len () as _ ; loop { stream . next_out = buf . as_mut_ptr () ; stream . avail_out = buf . len () as _ ; let err = unsafe { inflate (stream , InflateFlush :: Finish as _) } ; output . extend (& buf [.. buf . len () - stream . avail_out as usize]) ; match ReturnCode :: from (err) { ReturnCode :: BufError => { assert_eq ! (stream . avail_out , 0) ; stream . avail_out = buf . len () as _ ; } ReturnCode :: StreamEnd => break , other => panic ! ("unexpected {other:?}") , } } let err = inflateEnd (stream) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; output }) ; }
};
}
