// Generated macro for test_deflate_get_dict (function)
macro_rules! Depcrate_deflatetest_deflate_get_dict {
() => {
// Module: crate::deflate
// Provides: {"test_deflate_get_dict"}
// Dependencies: {}
# [test] fn test_deflate_get_dict () { assert_eq_rs_ng ! ({ let mut compr = [0u8 ; 1024] ; let mut strm = MaybeUninit :: zeroed () ; let ret = unsafe { deflateInit_ (strm . as_mut_ptr () , Z_BEST_COMPRESSION , zlibVersion () , core :: mem :: size_of ::< z_stream > () as _ ,) } ; let c_stream = unsafe { strm . assume_init_mut () } ; assert_eq ! (ReturnCode :: from (ret) , ReturnCode :: Ok) ; c_stream . avail_out = compr . len () as _ ; c_stream . next_out = compr . as_mut_ptr () ; let mut hello = * b"hello, hello!\0" ; c_stream . avail_in = hello . len () as _ ; c_stream . next_in = hello . as_mut_ptr () ; let ret = unsafe { deflate (c_stream , Z_FINISH) } ; assert_eq ! (ReturnCode :: from (ret) , ReturnCode :: StreamEnd) ; let mut dict_new = vec ! [0 ; 256] ; let mut dict_len = dict_new . len () as c_uint ; let ret = unsafe { deflateGetDictionary (c_stream , dict_new . as_mut_ptr () , & mut dict_len) } ; assert_eq ! (ReturnCode :: from (ret) , ReturnCode :: Ok) ; let dictionary = unsafe { CStr :: from_ptr (dict_new . as_ptr () . cast ()) } . to_owned () ; let ret = unsafe { deflateEnd (c_stream) } ; assert_eq ! (ReturnCode :: from (ret) , ReturnCode :: Ok) ; dictionary }) ; }
};
}
