// Generated macro for wrong_end_block (function)
macro_rules! Depcrate_parser_tests_errorswrong_end_block {
() => {
// Module: crate::parser::tests::errors
// Provides: {"wrong_end_block"}
// Dependencies: {}
# [test] fn wrong_end_block () { assert_err_msg ("{{ hey %}" , & ["1:9" , "expected an integer, a float, `true` or `false`, an identifier (must start with a-z), a square bracketed identifier (identifiers separated by `.` or `[]`s), or an expression"] ,) ; }
};
}
