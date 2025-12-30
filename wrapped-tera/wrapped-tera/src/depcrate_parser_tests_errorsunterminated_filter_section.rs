// Generated macro for unterminated_filter_section (function)
macro_rules! Depcrate_parser_tests_errorsunterminated_filter_section {
() => {
// Module: crate::parser::tests::errors
// Provides: {"unterminated_filter_section"}
// Dependencies: {}
# [test] fn unterminated_filter_section () { assert_err_msg (r#"{% filter uppercase %}sd"# , & ["1:25" , r#"expected tag or the filter section content"#] ,) ; }
};
}
