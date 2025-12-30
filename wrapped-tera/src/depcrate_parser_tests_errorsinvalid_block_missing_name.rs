// Generated macro for invalid_block_missing_name (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_block_missing_name {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_block_missing_name"}
// Dependencies: {}
# [test] fn invalid_block_missing_name () { assert_err_msg (r#"{% block %}"# , & ["1:10" , "expected an identifier (must start with a-z)"]) ; }
};
}
