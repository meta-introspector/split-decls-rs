// Generated macro for cve_test (function)
macro_rules! Depcrate_zlib_ng_cvecve_test {
() => {
// Module: crate::zlib_ng_cve
// Provides: {"cve_test"}
// Dependencies: {}
fn cve_test (input : & [u8]) { let mut output_ng = [0 ; 1 << 17] ; let config = DeflateConfig { window_bits : 15 , mem_level : 1 , .. DeflateConfig :: default () } ; let (output_ng , err) = compress_slice_ng (& mut output_ng , input , config) ; assert_eq ! (err , ReturnCode :: Ok) ; let mut output_rs = [0 ; 1 << 17] ; let (output_rs , err) = zlib_rs :: deflate :: compress_slice (& mut output_rs , input , config) ; assert_eq ! (err , ReturnCode :: Ok) ; assert_eq ! (output_ng , output_rs) ; let mut output = vec ! [0 ; input . len ()] ; let config = zlib_rs :: inflate :: InflateConfig { window_bits : 15 } ; let (output , err) = zlib_rs :: inflate :: uncompress_slice (& mut output , output_rs , config) ; assert_eq ! (err , ReturnCode :: Ok) ; assert_eq ! (input , output) ; }
};
}
