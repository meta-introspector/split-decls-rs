// Generated macro for version_error (function)
macro_rules! Depcrate_deflateversion_error {
() => {
// Module: crate::deflate
// Provides: {"version_error"}
// Dependencies: {}
# [test] fn version_error () { use libz_rs_sys :: { deflateInit_ , z_stream , zlibVersion , Z_OK , Z_VERSION_ERROR } ; let mut stream = core :: mem :: MaybeUninit :: zeroed () ; let ret = unsafe { deflateInit_ (stream . as_mut_ptr () , 1 , zlibVersion () , core :: mem :: size_of :: < z_stream > () as i32 ,) } ; assert_eq ! (ret , Z_OK) ; let ret = unsafe { deflateEnd (stream . as_mut_ptr ()) } ; assert_eq ! (ret , Z_OK) ; let ret = unsafe { deflateInit_ (stream . as_mut_ptr () , 1 , zlibVersion () , 1) } ; assert_eq ! (ret , Z_VERSION_ERROR) ; let ret = unsafe { deflateInit_ (stream . as_mut_ptr () , 1 , core :: ptr :: null () , core :: mem :: size_of :: < z_stream > () as i32 ,) } ; assert_eq ! (ret , Z_VERSION_ERROR) ; let ret = unsafe { deflateInit_ (stream . as_mut_ptr () , 1 , b"!\0" . as_ptr () as * const c_char , core :: mem :: size_of :: < z_stream > () as i32 ,) } ; assert_eq ! (ret , Z_VERSION_ERROR) ; let ret = unsafe { deflateInit2_ (stream . as_mut_ptr () , 1 , 0 , 0 , 0 , 0 , b"!\0" . as_ptr () as * const c_char , core :: mem :: size_of :: < z_stream > () as i32 ,) } ; assert_eq ! (ret , Z_VERSION_ERROR) ; }
};
}
