// Generated macro for deflate_bound_correct_basic (function)
macro_rules! Depcrate_deflatedeflate_bound_correct_basic {
() => {
// Module: crate::deflate
// Provides: {"deflate_bound_correct_basic"}
// Dependencies: {}
# [test] fn deflate_bound_correct_basic () { let config = DeflateConfig :: default () ; assert_deflate_bound_correct ((config , 42)) ; assert_deflate_bound_correct ((config , 123456)) ; assert_deflate_bound_correct ((config , 1 << 24)) ; }
};
}
