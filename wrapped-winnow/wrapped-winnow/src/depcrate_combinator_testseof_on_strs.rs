// Generated macro for eof_on_strs (function)
macro_rules! Depcrate_combinator_testseof_on_strs {
() => {
// Module: crate::combinator::tests
// Provides: {"eof_on_strs"}
// Dependencies: {}
# [test] fn eof_on_strs () { let not_over : & str = "Hello, world!" ; let is_over : & str = "" ; let res_not_over = eof . parse_peek (not_over) ; assert_parse ! (res_not_over , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "Hello, world!",
        },
    ),
)

"#]] . raw ()) ; let res_over = eof . parse_peek (is_over) ; assert_parse ! (res_over , str ! [[r#"
Ok(
    (
        "",
        "",
    ),
)

"#]] . raw ()) ; }
};
}
