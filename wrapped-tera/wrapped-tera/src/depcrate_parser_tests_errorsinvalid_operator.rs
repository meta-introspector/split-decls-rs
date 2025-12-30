// Generated macro for invalid_operator (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_operator {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_operator"}
// Dependencies: {}
# [test] fn invalid_operator () { assert_err_msg ("{{ hey =! }}" , & ["1:8" , "expected `or`, `and`, `not`, `<=`, `>=`, `<`, `>`, `==`, `!=`, `+`, `-`, `*`, `/`, `%`, a filter, or a variable end (`}}`)"] ,) ; }
};
}
