// Generated macro for unterminated_macro_call (function)
macro_rules! Depcrate_parser_tests_errorsunterminated_macro_call {
() => {
// Module: crate::parser::tests::errors
// Provides: {"unterminated_macro_call"}
// Dependencies: {}
# [test] fn unterminated_macro_call () { assert_err_msg ("{{ my::macro( }}" , & ["1:15" , "expected an identifier (must start with a-z)"]) ; }
};
}
