// Generated macro for invalid_include_no_string (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_include_no_string {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_include_no_string"}
// Dependencies: {}
# [test] fn invalid_include_no_string () { assert_err_msg ("{% include 1 %}" , & ["1:12" , "expected a string"]) ; }
};
}
