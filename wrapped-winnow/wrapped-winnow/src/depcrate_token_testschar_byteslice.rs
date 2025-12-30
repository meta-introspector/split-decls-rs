// Generated macro for char_byteslice (function)
macro_rules! Depcrate_token_testschar_byteslice {
() => {
// Module: crate::token::tests
// Provides: {"char_byteslice"}
// Dependencies: {}
# [test] fn char_byteslice () { fn f < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , char > { 'c' . parse_next (i) } let a = & b"abcd" [..] ; assert_parse ! (f . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    97,
                    98,
                    99,
                    100,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; let b = & b"cde" [..] ; assert_parse ! (f . parse_peek (Partial :: new (b)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                100,
                101,
            ],
            partial: true,
        },
        'c',
    ),
)

"#]] . raw ()) ; }
};
}
