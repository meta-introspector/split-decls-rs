// Generated macro for assert_deflate_bound_correct (function)
macro_rules! Depcrate_deflateassert_deflate_bound_correct {
() => {
// Module: crate::deflate
// Provides: {"assert_deflate_bound_correct"}
// Dependencies: {}
fn assert_deflate_bound_correct ((config , source_len) : (DeflateConfig , c_ulong)) { assert_eq_rs_ng ! ({ let mut strm = MaybeUninit :: zeroed () ; let err = deflateInit2_ (strm . as_mut_ptr () , config . level , config . method as i32 , config . window_bits , config . mem_level , config . strategy as i32 , VERSION , STREAM_SIZE ,) ; assert_eq ! (ReturnCode :: from (err) , ReturnCode :: Ok) ; let bound = deflateBound (strm . as_mut_ptr () , source_len) ; let ret = unsafe { deflateEnd (strm . as_mut_ptr ()) } ; assert_eq ! (ret , Z_OK) ; bound }) ; }
};
}
