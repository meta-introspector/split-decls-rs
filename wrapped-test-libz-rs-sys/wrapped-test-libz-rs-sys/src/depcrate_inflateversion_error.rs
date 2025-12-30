// Generated macro for version_error (function)
macro_rules! Depcrate_inflateversion_error {
() => {
// Module: crate::inflate
// Provides: {"version_error"}
// Dependencies: {}
# [test] fn version_error () { use libz_rs_sys :: * ; let mut stream = core :: mem :: MaybeUninit :: zeroed () ; let ret = unsafe { inflateInit_ (stream . as_mut_ptr () , zlibVersion () , core :: mem :: size_of :: < z_stream > () as i32 ,) } ; assert_eq ! (ret , Z_OK) ; let ret = unsafe { inflateEnd (stream . as_mut_ptr ()) } ; assert_eq ! (ret , Z_OK) ; let ret = unsafe { inflateInit_ (stream . as_mut_ptr () , zlibVersion () , 1) } ; assert_eq ! (ret , Z_VERSION_ERROR) ; let ret = unsafe { inflateInit_ (stream . as_mut_ptr () , core :: ptr :: null () , core :: mem :: size_of :: < z_stream > () as i32 ,) } ; assert_eq ! (ret , Z_VERSION_ERROR) ; let ret = unsafe { inflateInit_ (stream . as_mut_ptr () , b"!\0" . as_ptr () as * const c_char , core :: mem :: size_of :: < z_stream > () as i32 ,) } ; assert_eq ! (ret , Z_VERSION_ERROR) ; let ret = unsafe { inflateInit2_ (stream . as_mut_ptr () , 1 , b"!\0" . as_ptr () as * const c_char , core :: mem :: size_of :: < z_stream > () as i32 ,) } ; assert_eq ! (ret , Z_VERSION_ERROR) ; }
};
}
