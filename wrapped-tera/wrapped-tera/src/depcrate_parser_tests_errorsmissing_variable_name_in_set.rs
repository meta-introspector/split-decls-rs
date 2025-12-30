// Generated macro for missing_variable_name_in_set (function)
macro_rules! Depcrate_parser_tests_errorsmissing_variable_name_in_set {
() => {
// Module: crate::parser::tests::errors
// Provides: {"missing_variable_name_in_set"}
// Dependencies: {}
# [test] fn missing_variable_name_in_set () { assert_err_msg ("{% set = 1 %}" , & ["1:8" , "expected an identifier (must start with a-z)"]) ; }
};
}
