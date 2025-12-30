// Generated macro for issue_169 (function)
macro_rules! Depcrate_deflateissue_169 {
() => {
// Module: crate::deflate
// Provides: {"issue_169"}
// Dependencies: {}
# [test] fn issue_169 () { const INPUT : & str = include_str ! ("test-data/issue-169.js") ; let buf = INPUT ; crate :: assert_eq_rs_ng ! ({ let mut out = [0u8 ; 4096] ; let mut ret ; let mut strm = MaybeUninit :: zeroed () ; ret = deflateInit2_ (strm . as_mut_ptr () , Z_BEST_SPEED , Z_DEFLATED , - 15 , 8 , Z_DEFAULT_STRATEGY , VERSION , STREAM_SIZE ,) ; assert_eq ! (ret , Z_OK) ; let strm = strm . assume_init_mut () ; strm . avail_in = 2048 ; strm . avail_out = 1053 ; strm . next_in = buf . as_ptr () as * mut u8 ; strm . next_out = out . as_mut_ptr () ; ret = deflate (strm , Z_NO_FLUSH) ; assert_eq ! (ret , Z_OK) ; assert_eq ! (strm . avail_in , 0) ; assert_eq ! (strm . avail_out , 1053) ; assert_eq ! (strm . total_in , 2048) ; assert_eq ! (strm . total_out , 0) ; strm . avail_in = 67 ; ret = deflate (strm , Z_FINISH) ; assert_eq ! (ret , Z_OK) ; # [cfg (not (windows))] { assert_eq ! (strm . avail_in , 67) ; assert_eq ! (strm . avail_out , 0) ; assert_eq ! (strm . total_in , 2048) ; assert_eq ! (strm . total_out , 1053) ; } strm . avail_out = (out . len () as core :: ffi :: c_ulong - strm . total_out) as uInt ; ret = deflate (strm , Z_FINISH) ; # [cfg (not (windows))] { assert_eq ! (strm . avail_in , 0) ; assert_eq ! (strm . avail_out , 2854) ; assert_eq ! (strm . total_in , 2115) ; assert_eq ! (strm . total_out , 1242) ; } assert_eq ! (ret , Z_STREAM_END) ; assert_eq ! (strm . avail_in , 0) ; deflateEnd (strm) ; out }) ; }
};
}
