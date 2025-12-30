// Generated macro for unterminated_if_tag (function)
macro_rules! Depcrate_parser_tests_errorsunterminated_if_tag {
() => {
// Module: crate::parser::tests::errors
// Provides: {"unterminated_if_tag"}
// Dependencies: {}
# [test] fn unterminated_if_tag () { assert_err_msg (r#"{% if true %}sd"# , & ["1:16" , r#"expected tag or some content"#]) ; }
};
}
