// Generated macro for partial_is_not (function)
macro_rules! Depcrate_token_testspartial_is_not {
() => {
// Module: crate::token::tests
// Provides: {"partial_is_not"}
// Dependencies: {}
# [test] fn partial_is_not () { fn a_or_b < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { take_till (1 .. , ['a' , 'b']) . parse_next (i) } let a = Partial :: new (& b"cdab" [..]) ; assert_parse ! (a_or_b . parse_peek (a) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                97,
                98,
            ],
            partial: true,
        },
        [
            99,
            100,
        ],
    ),
)

"#]] . raw ()) ; let b = Partial :: new (& b"cbde" [..]) ; assert_parse ! (a_or_b . parse_peek (b) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                98,
                100,
                101,
            ],
            partial: true,
        },
        [
            99,
        ],
    ),
)

"#]] . raw ()) ; let c = Partial :: new (& b"abab" [..]) ; assert_parse ! (a_or_b . parse_peek (c) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    97,
                    98,
                    97,
                    98,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; let d = Partial :: new (& b"cdefba" [..]) ; assert_parse ! (a_or_b . parse_peek (d) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                98,
                97,
            ],
            partial: true,
        },
        [
            99,
            100,
            101,
            102,
        ],
    ),
)

"#]] . raw ()) ; let e = Partial :: new (& b"e" [..]) ; assert_parse ! (a_or_b . parse_peek (e) , str ! [[r#"
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
