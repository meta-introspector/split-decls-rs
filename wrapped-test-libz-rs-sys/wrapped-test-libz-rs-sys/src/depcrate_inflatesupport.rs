// Generated macro for support (function)
macro_rules! Depcrate_inflatesupport {
() => {
// Module: crate::inflate
// Provides: {"support"}
// Dependencies: {}
# [test] fn support () { use libz_rs_sys :: * ; let mut stream = mem_setup () ; let ret = unsafe { inflateInit_ (& mut stream , zlibVersion () , core :: mem :: size_of :: < z_stream > () as i32 ,) } ; assert_eq ! (ret , Z_OK) ; let ret = unsafe { inflateSetDictionary (& mut stream , std :: ptr :: null () , 0) } ; assert_eq ! (ret , Z_STREAM_ERROR) ; let ret = unsafe { inflateEnd (& mut stream) } ; assert_eq ! (ret , Z_OK) ; mem_done (& mut stream) ; }
};
}
