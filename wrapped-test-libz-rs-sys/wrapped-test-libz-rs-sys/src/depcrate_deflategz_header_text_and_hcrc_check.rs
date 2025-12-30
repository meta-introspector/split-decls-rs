// Generated macro for gz_header_text_and_hcrc_check (function)
macro_rules! Depcrate_deflategz_header_text_and_hcrc_check {
() => {
// Module: crate::deflate
// Provides: {"gz_header_text_and_hcrc_check"}
// Dependencies: {}
# [test] fn gz_header_text_and_hcrc_check () { let config = DeflateConfig { level : 9 , method : Method :: Deflated , window_bits : 31 , mem_level : 3 , strategy : Strategy :: Fixed , } ; assert_eq_rs_ng ! ({ let mut strm = MaybeUninit :: zeroed () ; let err = deflateInit2_ (strm . as_mut_ptr () , config . level , config . method as i32 , config . window_bits , config . mem_level , config . strategy as i32 , zlibVersion () , core :: mem :: size_of ::< z_stream > () as _ ,) ; assert_eq ! (err , 0) ; let strm = strm . assume_init_mut () ; let empty_c_string = [0u8] ; let mut header = gz_header { text : - 42 , time : 0 , os : 0 , extra : core :: ptr :: null_mut () , extra_len : 0 , name : empty_c_string . as_ptr () as * mut _ , comment : empty_c_string . as_ptr () as * mut _ , hcrc : - 42 , xflags : 0 , extra_max : 0 , name_max : 0 , comm_max : 0 , done : 0 , } ; let _ = deflateSetHeader (strm , & mut header) ; let input = "\0\u{2}\0\0\0\0\0\0\0\u{10}" ; let mut output = [0u8 ; 32] ; strm . avail_in = input . len () as _ ; strm . avail_out = output . len () as _ ; strm . next_in = input . as_ptr () . cast_mut () ; strm . next_out = output . as_mut_ptr () ; let err = deflate (strm , DeflateFlush :: NoFlush as i32) ; assert_eq ! (err , 0) ; deflateEnd (strm) ; output }) ; }
};
}
