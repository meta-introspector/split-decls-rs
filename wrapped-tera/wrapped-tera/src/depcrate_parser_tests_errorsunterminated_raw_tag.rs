// Generated macro for unterminated_raw_tag (function)
macro_rules! Depcrate_parser_tests_errorsunterminated_raw_tag {
() => {
// Module: crate::parser::tests::errors
// Provides: {"unterminated_raw_tag"}
// Dependencies: {}
# [test] fn unterminated_raw_tag () { assert_err_msg (r#"{% raw %}sd"# , & ["1:12" , "expected tag"]) ; }
};
}
