// Generated macro for partial_take_while_m_n (function)
macro_rules! Depcrate_token_testspartial_take_while_m_n {
() => {
// Module: crate::token::tests
// Provides: {"partial_take_while_m_n"}
// Dependencies: {}
# [test] fn partial_take_while_m_n () { fn x < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { take_while (2 ..= 4 , AsChar :: is_alpha) . parse_next (i) } let a = & b"" [..] ; let b = & b"a" [..] ; let c = & b"abc" [..] ; let d = & b"abc123" [..] ; let e = & b"abcde" [..] ; let f = & b"123" [..] ; assert_parse ! (x . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            2,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (x . parse_peek (Partial :: new (b)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (x . parse_peek (Partial :: new (c)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (x . parse_peek (Partial :: new (d)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                49,
                50,
                51,
            ],
            partial: true,
        },
        [
            97,
            98,
            99,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (x . parse_peek (Partial :: new (e)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                101,
            ],
            partial: true,
        },
        [
            97,
            98,
            99,
            100,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (x . parse_peek (Partial :: new (f)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    49,
                    50,
                    51,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; }
};
}
