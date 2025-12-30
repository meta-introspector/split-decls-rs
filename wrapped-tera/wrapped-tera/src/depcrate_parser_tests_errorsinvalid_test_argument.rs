// Generated macro for invalid_test_argument (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_test_argument {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_test_argument"}
// Dependencies: {}
# [test] fn invalid_test_argument () { assert_err_msg (r#"{% if a is odd(key=1) %}"# , & ["1:19" , "expected `or`, `and`, `not`, `<=`, `>=`, `<`, `>`, `==`, `!=`, `+`, `-`, `*`, `/`, `%`, or a filter"] ,) ; }
};
}
