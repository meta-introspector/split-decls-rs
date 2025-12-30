// Generated macro for uncompress_edge_cases (function)
macro_rules! Depcrate_inflateuncompress_edge_cases {
() => {
// Module: crate::inflate
// Provides: {"uncompress_edge_cases"}
// Dependencies: {}
# [test] fn uncompress_edge_cases () { let config = InflateConfig { window_bits : 15 } ; let (result , err) = uncompress_slice (& mut [] , & [] , config) ; assert_eq ! (err , ReturnCode :: DataError) ; assert ! (result . is_empty ()) ; let mut output = [0 ; 1] ; let (result , err) = uncompress_slice (& mut output , & [] , config) ; assert_eq ! (err , ReturnCode :: DataError) ; assert ! (result . is_empty ()) ; let input = b"Hello World!\n" ; let mut compressed = [0 ; 64] ; let (compressed , err) = compress_slice (& mut compressed , input , DeflateConfig :: new (6)) ; assert_eq ! (err , ReturnCode :: Ok) ; let (result , err) = uncompress_slice (& mut [] , compressed , config) ; assert_eq ! (err , ReturnCode :: DataError) ; assert ! (result . is_empty ()) ; }
};
}
