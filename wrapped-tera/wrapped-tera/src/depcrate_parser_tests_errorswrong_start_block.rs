// Generated macro for wrong_start_block (function)
macro_rules! Depcrate_parser_tests_errorswrong_start_block {
() => {
// Module: crate::parser::tests::errors
// Provides: {"wrong_start_block"}
// Dependencies: {}
# [test] fn wrong_start_block () { assert_err_msg ("{{ if true %}" , & ["1:7" , "expected `or`, `and`, `not`, `<=`, `>=`, `<`, `>`, `==`, `!=`, `+`, `-`, `*`, `/`, `%`, a filter, or a variable end (`}}`)"] ,) ; }
};
}
