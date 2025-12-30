// Generated macro for test_dict_deflate (function)
macro_rules! Depcrate_deflatetest_dict_deflate {
() => {
// Module: crate::deflate
// Provides: {"test_dict_deflate"}
// Dependencies: {}
# [test] fn test_dict_deflate () { let config = DeflateConfig { level : Z_BEST_COMPRESSION , .. Default :: default () } ; const DICTIONARY : & str = "hello" ; const HELLO : & str = "hello, hello!\0" ; assert_eq_rs_ng ! ({ let mut strm = MaybeUninit :: zeroed () ; let err = deflateInit2_ (strm . as_mut_ptr () , config . level , config . method as i32 , config . window_bits , config . mem_level , config . strategy as i32 , zlibVersion () , core :: mem :: size_of ::< z_stream > () as _ ,) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; let strm = strm . assume_init_mut () ; let err = deflateSetDictionary (strm , DICTIONARY . as_ptr () , DICTIONARY . len () as _) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; let dict_id = strm . adler ; let mut compr = [0 ; 32] ; strm . next_out = compr . as_mut_ptr () ; strm . avail_out = compr . len () as _ ; strm . next_in = HELLO . as_ptr () as * mut u8 ; strm . avail_in = HELLO . len () as _ ; let err = deflate (strm , DeflateFlush :: Finish as i32) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: StreamEnd) ; let err = deflateEnd (strm) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; (dict_id as c_ulong , compr) }) ; }
};
}
