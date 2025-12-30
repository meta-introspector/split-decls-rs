// Generated macro for deflate_bound_gzip_header_help (function)
macro_rules! Depcrate_deflatedeflate_bound_gzip_header_help {
() => {
// Module: crate::deflate
// Provides: {"deflate_bound_gzip_header_help"}
// Dependencies: {}
fn deflate_bound_gzip_header_help ((config , source_len , extra , name , comment) : (DeflateConfig , c_ulong , CString , CString , CString) ,) -> bool { let extra_len = extra . as_bytes () . len () ; let extra = extra . as_ptr () . cast_mut () . cast :: < u8 > () ; let name = name . as_ptr () . cast_mut () . cast :: < u8 > () ; let comment = comment . as_ptr () . cast_mut () . cast :: < u8 > () ; assert_eq_rs_ng ! ({ let mut strm = MaybeUninit :: zeroed () ; let err = deflateInit2_ (strm . as_mut_ptr () , config . level , config . method as i32 , config . window_bits , config . mem_level , config . strategy as i32 , zlibVersion () , core :: mem :: size_of ::< z_stream > () as _ ,) ; if err != 0 { return true ; } let mut header = gz_header { text : 0 , time : 0 , xflags : 0 , os : 0 , extra_len : extra_len as _ , extra , extra_max : 0 , name , name_max : 0 , comment , comm_max : 0 , hcrc : 1 , done : 0 , } ; let _ = deflateSetHeader (strm . as_mut_ptr () , & mut header) ; let bound = deflateBound (strm . as_mut_ptr () , source_len) ; deflateEnd (strm . as_mut_ptr ()) ; bound }) ; true }
};
}
