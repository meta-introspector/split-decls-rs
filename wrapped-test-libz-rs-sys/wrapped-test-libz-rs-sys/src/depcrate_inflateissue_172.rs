// Generated macro for issue_172 (function)
macro_rules! Depcrate_inflateissue_172 {
() => {
// Module: crate::inflate
// Provides: {"issue_172"}
// Dependencies: {}
# [test] fn issue_172 () { const BUF : & [u8] = & [31 , 139 , 8 , 0 , 0 , 0 , 0 , 0 , 0 , 3 , 75 , 173 , 40 , 72 , 77 , 46 , 73 , 77 , 81 , 200 , 47 , 45 , 41 , 40 , 45 , 1 , 0 , 176 , 1 , 57 , 179 , 15 , 0 , 0 , 0 ,] ; assert_eq_rs_ng ! ({ let mut exitcode = 0 ; for chunk in 1 .. BUF . len () { let mut ret ; let mut out = [0u8 ; 32] ; let mut strm = MaybeUninit :: zeroed () ; ret = inflateInit2_ (strm . as_mut_ptr () , 31 , VERSION , STREAM_SIZE) ; assert_eq ! (ret , Z_OK) ; let strm = strm . assume_init_mut () ; strm . avail_out = out . len () as _ ; strm . next_in = BUF . as_ptr () as * mut u8 ; strm . next_out = out . as_mut_ptr () ; while ret == Z_OK && strm . total_in < BUF . len () as _ { strm . avail_in = if chunk as c_ulong > (BUF . len () as c_ulong - strm . total_in) { (BUF . len () as c_ulong - strm . total_in) as c_uint } else { chunk as c_uint } ; ret = inflate (strm , Z_NO_FLUSH) ; } if ret != Z_STREAM_END { eprintln ! ("Finished with {ret} at chunk size {chunk}\n") ; exitcode = 1 ; } if & out [.. strm . total_out as usize] != b"expected output" { eprintln ! ("Output did not match at chunk size {chunk}\n") ; exitcode = 1 ; } let err = inflateEnd (strm) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; } assert ! (exitcode == 0) ; exitcode }) ; }
};
}
