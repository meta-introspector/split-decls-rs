// Generated macro for partial_take_while0 (function)
macro_rules! Depcrate_token_testspartial_take_while0 {
() => {
// Module: crate::token::tests
// Provides: {"partial_take_while0"}
// Dependencies: {}
# [test] fn partial_take_while0 () { fn f < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { take_while (0 .. , AsChar :: is_alpha) . parse_next (i) } let a = & b"" [..] ; let b = & b"abcd" [..] ; let c = & b"abcd123" [..] ; let d = & b"123" [..] ; assert_parse ! (f . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new (b)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new (c)) , str ! [[r#"
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
            100,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new (d)) , str ! [[r#"
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
        [],
    ),
)

"#]] . raw ()) ; }
};
}
