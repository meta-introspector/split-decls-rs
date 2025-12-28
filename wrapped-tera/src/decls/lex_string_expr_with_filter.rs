macro_rules! lex_string_expr_with_filter {
    () => {
        # [test] fn lex_string_expr_with_filter () { let inputs = vec ! [r#""hey" | capitalize"# , r#""hey""# , r#""hey" ~ 'ho' | capitalize"# , r#""hey" ~ ho | capitalize"# , r#"ho ~ ho ~ ho | capitalize"# , r#"ho ~ 'ho' ~ ho | capitalize"# , r#"ho ~ 'ho' ~ ho"# ,] ; for i in inputs { assert_lex_rule ! (Rule :: string_expr_filter , i) ; } }
    };
}

lex_string_expr_with_filter!();