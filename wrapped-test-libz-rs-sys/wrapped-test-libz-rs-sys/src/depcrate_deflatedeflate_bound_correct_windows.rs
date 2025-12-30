// Generated macro for deflate_bound_correct_windows (function)
macro_rules! Depcrate_deflatedeflate_bound_correct_windows {
() => {
// Module: crate::deflate
// Provides: {"deflate_bound_correct_windows"}
// Dependencies: {}
# [test] fn deflate_bound_correct_windows () { let config = DeflateConfig { level : 9 , method : Method :: Deflated , window_bits : - 13 , mem_level : 5 , strategy : Strategy :: Filtered , } ; let source_len = 4294967233 ; assert_deflate_bound_correct ((config , source_len)) ; let config = DeflateConfig { level : 0 , method : Method :: Deflated , window_bits : 15 , mem_level : 5 , strategy : Strategy :: HuffmanOnly , } ; let source_len = 4294967289 ; assert_deflate_bound_correct ((config , source_len)) ; }
};
}
