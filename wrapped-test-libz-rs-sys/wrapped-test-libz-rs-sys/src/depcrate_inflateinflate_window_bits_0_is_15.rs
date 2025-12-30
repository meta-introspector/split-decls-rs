// Generated macro for inflate_window_bits_0_is_15 (function)
macro_rules! Depcrate_inflateinflate_window_bits_0_is_15 {
() => {
// Module: crate::inflate
// Provides: {"inflate_window_bits_0_is_15"}
// Dependencies: {}
# [test] fn inflate_window_bits_0_is_15 () { let input = b"Hello World!\n" ; let mut compressed = [0 ; 64] ; let (compressed , err) = compress_slice (& mut compressed , input , DeflateConfig :: new (6)) ; assert_eq ! (err , ReturnCode :: Ok) ; let config = InflateConfig { window_bits : 15 } ; let mut output_15 = [0 ; 64] ; let (output_15 , err) = uncompress_slice (& mut output_15 , compressed , config) ; assert_eq ! (err , ReturnCode :: Ok) ; let config = InflateConfig { window_bits : 0 } ; let mut output_0 = [0 ; 64] ; let (output_0 , err) = uncompress_slice (& mut output_0 , compressed , config) ; assert_eq ! (err , ReturnCode :: Ok) ; assert_eq ! (output_15 , output_0) ; assert_eq ! (output_15 , input) ; }
};
}
