// Generated macro for partial_take_till1 (function)
macro_rules! Depcrate_token_testspartial_take_till1 {
() => {
// Module: crate::token::tests
// Provides: {"partial_take_till1"}
// Dependencies: {}
# [test] fn partial_take_till1 () { fn f < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { take_till (1 .. , AsChar :: is_alpha) . parse_next (i) } let a = & b"" [..] ; let b = & b"abcd" [..] ; let c = & b"123abcd" [..] ; let d = & b"123" [..] ; assert_parse ! (f . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new (b)) , str ! [[r#"
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

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new (c)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                97,
                98,
                99,
                100,
            ],
            partial: true,
        },
        [
            49,
            50,
            51,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new (d)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; }
};
}
