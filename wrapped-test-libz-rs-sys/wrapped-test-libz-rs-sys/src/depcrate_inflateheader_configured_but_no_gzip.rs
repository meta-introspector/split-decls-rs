// Generated macro for header_configured_but_no_gzip (function)
macro_rules! Depcrate_inflateheader_configured_but_no_gzip {
() => {
// Module: crate::inflate
// Provides: {"header_configured_but_no_gzip"}
// Dependencies: {}
# [test] fn header_configured_but_no_gzip () { let input : & [u8] = & [0x63 , 0x18 , 0x68 , 0x30 , 0xd0 , 0x0 , 0x0] ; assert_eq_rs_ng ! ({ let mut stream = MaybeUninit ::< z_stream >:: zeroed () ; let init_err = unsafe { inflateInit2_ (stream . as_mut_ptr () , 47 , VERSION , STREAM_SIZE) } ; assert_eq ! (init_err , Z_OK) ; let stream = stream . assume_init_mut () ; let mut header = gz_header { text : 0 , time : 0 , xflags : 0 , os : 0 , extra : core :: ptr :: null_mut () , extra_len : 0 , extra_max : 0 , name : core :: ptr :: null_mut () , name_max : 0 , comment : core :: ptr :: null_mut () , comm_max : 0 , hcrc : 0 , done : 123 , } ; let ret = unsafe { inflateGetHeader (stream , & mut header) } ; assert_eq ! (ReturnCode :: from (ret) , ReturnCode :: Ok) ; stream . avail_in = input . len () as _ ; stream . next_in = input . as_ptr () as * mut _ ; let mut out = [0u8 ; 32] ; stream . avail_out = out . len () as _ ; stream . next_out = out . as_mut_ptr () ; let ret = unsafe { inflate (stream , InflateFlush :: NoFlush as _) } ; assert_eq ! (ReturnCode :: from (ret) , ReturnCode :: DataError) ; let ret = unsafe { inflateEnd (stream) } ; assert_eq ! (ret , Z_OK) ; assert_eq ! (header . done , - 1) ; }) }
};
}
