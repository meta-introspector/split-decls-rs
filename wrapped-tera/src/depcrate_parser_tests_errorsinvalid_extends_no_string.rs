// Generated macro for invalid_extends_no_string (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_extends_no_string {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_extends_no_string"}
// Dependencies: {}
# [test] fn invalid_extends_no_string () { assert_err_msg ("{% extends 1 %}" , & ["1:12" , "expected a string"]) ; }
};
}
