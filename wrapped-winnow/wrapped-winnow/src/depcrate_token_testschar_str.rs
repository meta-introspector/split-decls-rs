// Generated macro for char_str (function)
macro_rules! Depcrate_token_testschar_str {
() => {
// Module: crate::token::tests
// Provides: {"char_str"}
// Dependencies: {}
# [test] fn char_str () { fn f < 'i > (i : & mut Partial < & 'i str >) -> TestResult < Partial < & 'i str > , char > { 'c' . parse_next (i) } let a = "abcd" ; assert_parse ! (f . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "abcd",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; let b = "cde" ; assert_parse ! (f . parse_peek (Partial :: new (b)) , str ! [[r#"
Ok(
    (
        Partial {
            input: "de",
            partial: true,
        },
        'c',
    ),
)

"#]] . raw ()) ; }
};
}
