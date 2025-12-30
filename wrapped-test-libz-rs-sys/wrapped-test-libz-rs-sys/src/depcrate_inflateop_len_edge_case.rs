// Generated macro for op_len_edge_case (function)
macro_rules! Depcrate_inflateop_len_edge_case {
() => {
// Module: crate::inflate
// Provides: {"op_len_edge_case"}
// Dependencies: {}
# [test] fn op_len_edge_case () { let window_bits = - 9 ; let input = & include_bytes ! ("test-data/op-len-edge-case.zraw") ; assert_eq_rs_ng ! ({ let mut output : Vec < u8 > = Vec :: with_capacity (1 << 15) ; let mut buf = [0 ; 266] ; let mut stream = MaybeUninit ::< z_stream >:: zeroed () ; let err = unsafe { inflateInit2_ (stream . as_mut_ptr () , window_bits , zlibVersion () , core :: mem :: size_of ::< z_stream > () as c_int ,) } ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; let stream = unsafe { stream . assume_init_mut () } ; stream . next_in = input . as_ptr () as * mut u8 ; stream . avail_in = input . len () as _ ; while stream . avail_in != 0 { stream . next_out = buf . as_mut_ptr () ; stream . avail_out = buf . len () as _ ; let err = unsafe { inflate (stream , InflateFlush :: NoFlush as _) } ; if ReturnCode :: from (err) == ReturnCode :: BufError { output . extend (& buf [.. stream . avail_out as usize]) ; stream . avail_out = buf . len () as _ ; continue ; } } let err = inflateEnd (stream) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; output }) ; }
};
}
