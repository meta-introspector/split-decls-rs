// Generated macro for partial_is_a (function)
macro_rules! Depcrate_token_testspartial_is_a {
() => {
// Module: crate::token::tests
// Provides: {"partial_is_a"}
// Dependencies: {}
# [test] fn partial_is_a () { fn a_or_b < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { take_while (1 .. , ['a' , 'b']) . parse_next (i) } let a = Partial :: new (& b"abcd" [..]) ; assert_parse ! (a_or_b . parse_peek (a) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                99,
                100,
            ],
            partial: true,
        },
        [
            97,
            98,
        ],
    ),
)

"#]] . raw ()) ; let b = Partial :: new (& b"bcde" [..]) ; assert_parse ! (a_or_b . parse_peek (b) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                99,
                100,
                101,
            ],
            partial: true,
        },
        [
            98,
        ],
    ),
)

"#]] . raw ()) ; let c = Partial :: new (& b"cdef" [..]) ; assert_parse ! (a_or_b . parse_peek (c) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    99,
                    100,
                    101,
                    102,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; let d = Partial :: new (& b"bacdef" [..]) ; assert_parse ! (a_or_b . parse_peek (d) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                99,
                100,
                101,
                102,
            ],
            partial: true,
        },
        [
            98,
            97,
        ],
    ),
)

"#]] . raw ()) ; }
};
}
