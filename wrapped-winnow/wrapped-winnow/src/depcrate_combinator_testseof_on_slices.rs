// Generated macro for eof_on_slices (function)
macro_rules! Depcrate_combinator_testseof_on_slices {
() => {
// Module: crate::combinator::tests
// Provides: {"eof_on_slices"}
// Dependencies: {}
# [test] fn eof_on_slices () { let not_over : & [u8] = & b"Hello, world!" [..] ; let is_over : & [u8] = & b"" [..] ; let res_not_over = eof . parse_peek (not_over) ; assert_parse ! (res_not_over , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: [
                72,
                101,
                108,
                108,
                111,
                44,
                32,
                119,
                111,
                114,
                108,
                100,
                33,
            ],
        },
    ),
)

"#]] . raw ()) ; let res_over = eof . parse_peek (is_over) ; assert_parse ! (res_over , str ! [[r#"
Ok(
    (
        [],
        [],
    ),
)

"#]] . raw ()) ; }
};
}
