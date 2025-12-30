// Generated macro for lex_string_expr_with_filter (function)
macro_rules! Depcrate_parser_tests_lexerlex_string_expr_with_filter {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_string_expr_with_filter"}
// Dependencies: {}
# [test] fn lex_string_expr_with_filter () { let inputs = vec ! [r#""hey" | capitalize"# , r#""hey""# , r#""hey" ~ 'ho' | capitalize"# , r#""hey" ~ ho | capitalize"# , r#"ho ~ ho ~ ho | capitalize"# , r#"ho ~ 'ho' ~ ho | capitalize"# , r#"ho ~ 'ho' ~ ho"# ,] ; for i in inputs { assert_lex_rule ! (Rule :: string_expr_filter , i) ; } }
};
}
