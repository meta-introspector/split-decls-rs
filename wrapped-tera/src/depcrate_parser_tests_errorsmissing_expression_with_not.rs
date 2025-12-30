// Generated macro for missing_expression_with_not (function)
macro_rules! Depcrate_parser_tests_errorsmissing_expression_with_not {
() => {
// Module: crate::parser::tests::errors
// Provides: {"missing_expression_with_not"}
// Dependencies: {}
# [test] fn missing_expression_with_not () { assert_err_msg ("{% if not %}" , & ["1:11" , "expected an expression"]) ; }
};
}
