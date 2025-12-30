// Generated macro for inflate_get_header_non_gzip_stream (function)
macro_rules! Depcrate_inflateinflate_get_header_non_gzip_stream {
() => {
// Module: crate::inflate
// Provides: {"inflate_get_header_non_gzip_stream"}
// Dependencies: {}
# [test] fn inflate_get_header_non_gzip_stream () { use libz_rs_sys :: * ; let mut stream = mem_setup () ; let win = 15 ; let init_err = unsafe { inflateInit2_ (& mut stream , win , VERSION , STREAM_SIZE) } ; if init_err != Z_OK { mem_done (& mut stream) ; return ; } let mut header = gz_header :: default () ; assert_eq ! (unsafe { inflateGetHeader (& mut stream , & mut header) } , ReturnCode :: StreamError as i32) ; let ret = unsafe { inflateEnd (& mut stream) } ; assert_eq ! (ret , Z_OK) ; mem_done (& mut stream) ; }
};
}
