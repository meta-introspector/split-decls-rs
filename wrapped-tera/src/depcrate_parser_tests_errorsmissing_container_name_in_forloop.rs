// Generated macro for missing_container_name_in_forloop (function)
macro_rules! Depcrate_parser_tests_errorsmissing_container_name_in_forloop {
() => {
// Module: crate::parser::tests::errors
// Provides: {"missing_container_name_in_forloop"}
// Dependencies: {}
# [test] fn missing_container_name_in_forloop () { assert_err_msg ("{% for i in %}" , & ["1:13" , "expected an expression or an array of values"]) ; }
};
}
